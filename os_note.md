# Lec01

1. Multi levels of Monolithic OS. Each layer only uses service from lower layer
2. Micro-kernel: move kernel functionalities to user space, and each user module communicate in message passing. It is safe and flexible, but its performance is low.
3. Exokernel: kernel allocated physical resources for applications, and applications decides how to use.

# Lec02

1. Isolation is an important concept.
2. Two approaches to Isolation: address space & privilege mode/ interrupt mechanism

3. The applications should not access to sensitive CPU registers and devices

4. As to address space, virtual memory is introduced.
5. CPU hardware support different privilege mode: Kernel / User Mode.
6. Kernel mode can access to external devices, configure address spaces, and read and write special system-level registers.
7. OS kernel runs in Kernel mode and normal applications run in user mode.
8.  CPU hardware should support interrupt/exception.
9. Interrupt is asynchronous, caused by signal from external I/O devices
10. When interrupted, processor save context and invoke interrupt handler. After that, context is switched back.
11. This mechanism allow OS kernel to manage resources periodically.
12. Qemu support RV64 ISA with MMU/TLB

13. Boot sequence: 

    ```mermaid
    graph LR
    	A[CPU/Regs]-->B[Memory]
    	B-->C[Basic devices]
    	C-->D[Execute ROM code]
    ```


# Lec03

1. System call: request from application to OS to get OS services.
2. Exception: request caused by illegal instruction or failed instruction
3. Hardware interrupt: request from external devices.
4. Four privilege level: U S H M
5. OS kernel is trusted third party, and it can execute privileged instruction and manage the hardware. In general, it provides us with various of service.

6. Source of interrupt: devices

7. Source of exception and system call: application

8. Response to interrupt is asynchronous, different from  exception. Response to system call can be both of them.

9. Interrupt is transparent to application.

10. Exception will cause applications to be killed or re-executed.

11. Two unit dealt with interrupt: CLINT(core local interrupter) & PLIC(platform-level interrupt controller)

12. Three standard interrupt sources: software, timer, external devices

13. `sstatus` & `sie` are CSR (control status registers)

    