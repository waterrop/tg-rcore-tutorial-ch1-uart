# tg-rcore-tutorial-ch1-uart 代码实现计划

## 1. 目标与范围

- 在 `tg-rcore-tutorial-ch1` 基础上扩展出 `tg-rcore-tutorial-ch1-uart` 内核 crate。
- 新建 `tg-rcore-tutorial-uart` 功能组件 crate，提供 S-Mode 串口驱动能力。
- 将当前基于 `SBI console_putchar` 的输出路径替换为 UART MMIO 轮询输出。
- 保持最小可运行目标：QEMU `virt` 平台串口输出 `Hello, world!`，随后正常关机。
- 补充一份面向实验教学的实现与验证文档。

## 2. 总体设计

- **内核 crate（tg-rcore-tutorial-ch1-uart）**
  - 负责启动入口 `_start`、栈初始化、`rust_main` 主流程、panic 收口。
  - 输出接口从直接调用 SBI 改为调用 `tg-rcore-tutorial-uart` 提供的写字符接口。
  - 关机路径可暂保留现有 `shutdown`（不影响“输出经 UART”目标）。

- **功能组件 crate（tg-rcore-tutorial-uart）**
  - 提供 `uart_putc(u8)`、`uart_puts(&[u8])` 等最小 API。
  - 内部封装 QEMU `virt` 平台 UART16550 访问（基址 `0x1000_0000`）。
  - 采用轮询方式发送：检查发送保持寄存器空闲后写入字节。

## 3. 分阶段实施

### 阶段 A：工程结构调整

1. 在仓库中新增 `tg-rcore-tutorial-uart` crate。
2. 配置 workspace 或路径依赖，使内核 crate 能引用 UART crate。
3. 统一 `Cargo.toml` 的包名、依赖与目标三元组设置。

### 阶段 B：UART 驱动实现

1. 在 UART crate 中定义寄存器偏移与 MMIO 读写原语。
2. 实现发送轮询逻辑（LSR 检查 + THR 写入）。
3. 封装安全边界：对外提供最小安全接口，对内保留 `unsafe` 访问。
4. 增加基础单元测试或主机桩模块，保证非 riscv64 下可编译。

### 阶段 C：内核输出路径切换

1. `src/main.rs` 去除 `console_putchar` 直接调用。
2. `rust_main` 改为通过 UART crate 输出字符串。
3. `panic_handler` 按需补充 panic 文本输出后再关机。
4. 保持 `_start -> rust_main -> shutdown` 控制流不变。

### 阶段 D：构建与运行验证

1. 交叉编译：`cargo build`。
2. 运行验证：`cargo run`，观察串口输出是否正确。
3. 验证关机场景：正常路径与 panic 路径至少各一次。
4. 确认在主机平台检查命令下可通过基础编译。

## 4. 关键实现点

- UART16550 常用寄存器：
  - THR/RBR: `base + 0x00`
  - LSR: `base + 0x05`
  - THRE 位：`LSR[5]`
- 轮询发送伪流程：
  1. 循环读取 LSR。
  2. 当 THRE=1 时向 THR 写入待发字节。
  3. 返回继续下一个字节。
- 字符串输出时处理 `\n`，必要时转换为 `\r\n` 以提升终端兼容性。

## 5. 交付物

1. `tg-rcore-tutorial-ch1-uart` 内核 crate（可运行）。
2. `tg-rcore-tutorial-uart` 功能组件 crate（可复用）。
3. 输出路径从 SBI 输出切换到 UART 轮询输出的代码改动。
4. 实验指导文档（背景、步骤、验证、常见问题）。

## 6. 验收标准（DoD）

- 在 QEMU 中看到由 UART 输出的 `Hello, world!`。
- 代码中不再使用 `SBI console_putchar` 作为主输出路径。
- 关机行为正确，程序可自动退出。
- 文档能够指导他人复现实验并理解关键代码路径。
