# question1
扩展tg-rcore-tutorial-ch1 内核crate，形成 tg-rcore-tutorial-ch1-uart 极简内核crate，创建 tg-rcore-tutorial-uart功能组件crate，让ch1内核不通过sbi输出字符，而是通过实现S-Mode的串口驱动来输出字符以轮询的方式。并形成一个对tg-rcore-tutorial-ch1-uart 内核crate，tg-rcore-tutorial-uart功能组件crate的实验指导文档。
要求：
1.将tg-rcore-rcore-tutorial-ch1-uart的代码实现计划保存到/home/hdu/study/rust/2026s-ai4ose-lab/tg-rcore-tutorial-ch1-uart/docs/uart_code_plan.md里

# res1
AI生成了代码实现计划看着没什么问题，让AI实现。

# question2
AI完成代码实现成功完成功能。

# question3
现在你作为一个老师，我是学生，为我生成一份关于刚刚所实现的这个ch1-uart内核和uart组件的有关知识问题让我回答，要求问题要详细，要覆盖到代码实现计划里的所有内容。生成的问题要包含以下内容：
1. 串口驱动的实现原理
2. 轮询方式的实现原理
3. 如何在ch1内核中使用uart组件输出字符
4. 保存在/home/hdu/study/rust/2026s-ai4ose-lab/tg-rcore-tutorial-ch1-uart/docs/uart_guide.md里

# question4
我正在学习使用rust写一个串口驱动程序，但是不知道如何开始，请为我提供一个简单的实现计划。
要求：
1.给出操作系统里关于串口驱动的理论知识。
2.给出riscv架构中与串口驱动有关的规范和寄存器，并给出相关的寄存器的名字、字段、及作用。
3.给出rust里该如何实现。
4.给出从操作系统理论到riscv架构的规范到rust的代码实现，这一主要过程是如何实现的。

# question5
将/home/hdu/study/rust/2026s-ai4ose-lab/tg-rcore-tutorial-ch1-uart/docs/uart_guide.md的内容读取并按要求完成任务。
要求：
1.给出每个问题的标准答案。
2.给出对我的答案的评价，不得删除我的答案。
3.如果我的答案有错误，给出错误的原因。
4.如果我的答案有遗漏，给出遗漏的原因。
5.将这次任务的内容保存到/home/hdu/study/rust/2026s-ai4ose-lab/tg-rcore-tutorial-ch1-uart/docs/uart_answer.md里。