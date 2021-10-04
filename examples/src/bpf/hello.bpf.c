#include "vmlinux.h"
#include <bpf/bpf_helpers.h>


SEC("tracepoint/syscalls/sys_enter_execve")
int hello_execve() {
	u64 pid_tgid = bpf_get_current_pid_tgid();
	bpf_printk("exec pid= %d\n",pid_tgid >> 32);

    return 0;
}

char __license[] SEC("license") = "Dual MIT/GPL";
