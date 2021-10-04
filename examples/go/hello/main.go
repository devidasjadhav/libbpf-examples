//go:build linux
// +build linux

// This program demonstrates attaching an eBPF program to a kernel symbol.
// The eBPF program will be attached to the start of the sys_execve
// kernel function and prints out the number of times it has been called
// every second.
package main

import (
	"fmt"

	"github.com/cilium/ebpf/link"
	"golang.org/x/sys/unix"
)

//go:generate go run github.com/cilium/ebpf/cmd/bpf2go -cc clang-11 HelloExample ../../src/bpf/hello.bpf.c -- -nostdinc -I../headers

const mapKey uint32 = 0

func setLimit() {
	// Increase the rlimit of the current process to provide sufficient space
	// for locking memory for the eBPF map.
	if err := unix.Setrlimit(unix.RLIMIT_MEMLOCK, &unix.Rlimit{
		Cur: unix.RLIM_INFINITY,
		Max: unix.RLIM_INFINITY,
	}); err != nil {
		fmt.Println("failed to set temporary rlimit: %v", err)
	}
}


func main() {

	// This loads generated files to objs
	//to be used to refer functions maps etc deined in bpf.c file

	objs := HelloExampleObjects{}
	if err := LoadHelloExampleObjects(&objs, nil); err != nil {
		fmt.Println("loading objects: %v", err)
	}
	defer objs.Close()

	// link kprobe with bpf program
	//here we are linking sys_execve with function deined in bpf.c
	kp, err := link.Tracepoint("syscalls", "sys_enter_execve", objs.HelloExecve)
//	kp, err := link.Kprobe("sys_execve", objs.HelloExecve)
	if err != nil {
		fmt.Println("opening hello: %s", err)
	}
	defer kp.Close()

	// Read loop reporting the total amount of times the kernel
	// function was entered, once per second.


	fmt.Println("Waiting for ^c..")
	for {
	}
}
