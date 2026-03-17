# RISC-V 架构下 Rust 串口驱动开发完整指南
## 文档说明
本文档面向 Rust 嵌入式/操作系统开发新手，完整覆盖**串口驱动理论知识 → RISC-V 串口硬件规范 → Rust 代码实现**全流程，基于 RISC-V 标准 16550 兼容 UART 串口，提供可直接运行的最简轮询模式驱动实现。

---

## 目录
1. 操作系统串口驱动核心理论
2. RISC-V 16550 UART 串口规范与寄存器详解
3. Rust 串口驱动完整实现（裸机版）
4. 从理论到硬件到代码的实现链路解析

---

# 1. 操作系统串口驱动核心理论
## 1.1 串口基础概念
串口（UART/USART）是**异步串行通信接口**，是操作系统最基础的字符设备，主要用于内核日志打印、调试、与外部设备通信。

## 1.2 核心设备模型
串口属于**内存映射 I/O（MMIO）设备**：
硬件寄存器被映射到物理地址空间，操作系统/内核无需特殊指令，**直接读写对应内存地址**即可操作硬件。

## 1.3 驱动工作模式
- **轮询模式（Polling）**：入门首选，循环检查硬件状态寄存器，等待设备就绪后执行收发操作，实现简单。
- **中断模式（Interrupt）**：进阶高效模式，数据就绪时触发硬件中断，内核通过中断处理函数响应，无需持续轮询。

## 1.4 驱动核心功能
1. 串口初始化（波特率、数据位、校验位、停止位配置）
2. 发送单个字符/字符串
3. 接收单个字符
4. 硬件状态检测（发送就绪、接收就绪）

## 1.5 操作系统驱动职责
屏蔽底层硬件细节，向上层提供统一的字符读写接口（如 `print`/`getchar`），向下直接操作硬件寄存器。

---

# 2. RISC-V 16550 UART 串口规范与寄存器详解
RISC-V 标准开发板（QEMU Virt、K210、VisionFive）均使用 **16550 兼容 UART** 串口，寄存器布局固定。

## 2.1 基础硬件参数
- MMIO 基地址（QEMU RISC-V Virt）：`0x10000000`
- 寄存器位宽：每个寄存器占 **1 字节**
- 寄存器数量：8 个核心寄存器（偏移 0x00~0x07）
- 波特率计算公式：`波特率除数 = 时钟频率 / (16 × 目标波特率)`
- QEMU 默认时钟：3.6864MHz，9600 波特率对应除数 = 24

## 2.2 核心寄存器详情
| 寄存器名 | 偏移地址 | 访问模式 | 核心字段/功能 |
|----------|----------|----------|---------------|
| RBR（接收缓冲寄存器） | 0x00 | 只读 | DLAB=0 时，存放接收到的字节数据 |
| THR（发送保持寄存器） | 0x00 | 只写 | DLAB=0 时，写入待发送的字节数据 |
| IER（中断使能寄存器） | 0x01 | 读写 | DLAB=0 时，使能/关闭接收/发送中断 |
| IIR（中断标识寄存器） | 0x02 | 只读 | 标识当前触发的中断类型 |
| LCR（线路控制寄存器） | 0x03 | 读写 | 核心配置：数据位、停止位、校验位、DLAB 位 |
| MCR（调制解调器控制寄存器） | 0x04 | 读写 | 硬件流控控制（入门可忽略） |
| LSR（线路状态寄存器） | 0x05 | 只读 | 核心状态：接收就绪、发送缓冲区空、错误状态 |
| SCR（暂存寄存器） | 0x07 | 读写 | 临时数据存储 |

## 2.3 关键寄存器字段（必记）
1. **LCR.7（DLAB 位）**
   - 0：访问 RBR/THR/IER 寄存器
   - 1：访问波特率除数寄存器（DLL/DLM）
2. **LCR[1:0]（数据位配置）**
   - `0b11`：8 位数据位（标准通信配置）
3. **LSR.0（DR 位）**
   - 1：接收缓冲区有数据，可读取
4. **LSR.5（THRE 位）**
   - 1：发送缓冲区为空，可写入

---

# 3. Rust 串口驱动完整实现（裸机版）
## 3.1 环境要求
1. Rust 嵌入式工具链：`rustup target add riscv64gc-unknown-none-elf`
2. 运行环境：裸机/自定义操作系统，禁用 Rust 标准库

## 3.2 完整代码实现
```rust
#![no_std]
#![no_main]

// 内核 panic 处理函数（裸机开发必备）
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ====================== 硬件常量定义 ======================
/// UART MMIO 基地址（QEMU RISC-V Virt 平台）
const UART_BASE: usize = 0x10000000;
/// 9600 波特率对应的除数
const BAUD_DIV: u8 = 24;

/// UART 寄存器偏移枚举
#[derive(Clone, Copy)]
enum UartReg {
    RBR_THR = 0x00, // 接收/发送共用寄存器
    IER = 0x01,     // 中断使能寄存器
    LCR = 0x03,     // 线路控制寄存器
    LSR = 0x05,     // 线路状态寄存器
}

// LCR 寄存器配置常量
const LCR_DLAB: u8 = 1 << 7; // 除数锁存访问位
const LCR_8BIT: u8 = 0b11;    // 8 位数据位配置

// LSR 寄存器状态常量
const LSR_DR: u8 = 1 << 0;   // 接收数据就绪标志位
const LSR_THRE: u8 = 1 << 5; // 发送缓冲区空标志位

// ====================== 寄存器读写工具函数 ======================
/// 读取 UART 寄存器
unsafe fn uart_read(reg: UartReg) -> u8 {
    core::ptr::read_volatile((UART_BASE + reg as usize) as *const u8)
}

/// 写入 UART 寄存器
unsafe fn uart_write(reg: UartReg, val: u8) {
    core::ptr::write_volatile((UART_BASE + reg as usize) as *mut u8, val);
}

// ====================== 串口初始化函数 ======================
/// 初始化 UART：配置波特率、数据格式、关闭中断
pub unsafe fn uart_init() {
    // 1. 设置 DLAB=1，允许配置波特率
    uart_write(UartReg::LCR, LCR_DLAB);
    // 2. 写入波特率除数（低 8 位 + 高 8 位）
    uart_write(UartReg::RBR_THR, BAUD_DIV);
    uart_write(UartReg::IER, BAUD_DIV >> 8);
    // 3. 设置 DLAB=0 + 8N1 通信格式（8 数据位+无校验+1 停止位）
    uart_write(UartReg::LCR, LCR_8BIT);
    // 4. 关闭所有中断（轮询模式）
    uart_write(UartReg::IER, 0);
}

// ====================== 核心收发函数 ======================
/// 发送单个字符
pub unsafe fn uart_putc(c: u8) {
    // 轮询等待发送缓冲区为空
    loop {
        if (uart_read(UartReg::LSR) & LSR_THRE) != 0 {
            break;
        }
    }
    // 写入数据到发送寄存器
    uart_write(UartReg::RBR_THR, c);
}

/// 发送字符串
pub unsafe fn uart_puts(s: &str) {
    for &c in s.as_bytes() {
        uart_putc(c);
    }
}

/// 阻塞接收单个字符
pub unsafe fn uart_getc() -> u8 {
    // 轮询等待接收数据就绪
    loop {
        if (uart_read(UartReg::LSR) & LSR_DR) != 0 {
            break;
        }
    }
    // 读取接收缓冲区数据
    uart_read(UartReg::RBR_THR)
}

// ====================== 程序入口函数 ======================
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        uart_init(); // 初始化串口
        uart_puts("Hello RISC-V UART from Rust!\n"); // 测试打印

        // 串口回显功能：接收字符并原样发送
        loop {
            let c = uart_getc();
            uart_putc(c);
        }
    }
}