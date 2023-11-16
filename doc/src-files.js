var srcIndex = JSON.parse('{\
"benchmark":["",[],["lib.rs"]],\
"echo":["",[],["lib.rs"]],\
"invalid_module":["",[],["lib.rs"]],\
"key_value_lookup":["",[],["lib.rs"]],\
"location_utils":["",[],["lib.rs"]],\
"lookup_data_checker":["",[],["main.rs"]],\
"lookup_data_generator":["",[],["data.rs","lib.rs"]],\
"micro_rpc":["",[],["lib.rs","status.rs"]],\
"micro_rpc_build":["",[],["lib.rs"]],\
"oak_channel":["",[],["client.rs","frame.rs","lib.rs","message.rs","server.rs"]],\
"oak_client":["",[],["lib.rs","transport.rs","verifier.rs"]],\
"oak_containers_hello_world_trusted_app":["",[],["app_service.rs","lib.rs","orchestrator_client.rs"]],\
"oak_containers_hello_world_untrusted_app":["",[],["app_client.rs","lib.rs"]],\
"oak_containers_launcher":["",[],["lib.rs","qemu.rs","server.rs"]],\
"oak_containers_orchestrator":["",[],["container_runtime.rs","dice.rs","ipc_server.rs","key_provisioning.rs","lib.rs","logging.rs","metrics.rs"]],\
"oak_containers_orchestrator_client":["",[],["lib.rs"]],\
"oak_containers_stage1":["",[],["client.rs","dice.rs","image.rs","main.rs"]],\
"oak_containers_syslogd":["",[],["log_relay.rs","main.rs","systemd_journal.rs"]],\
"oak_core":["",[],["lib.rs","samplestore.rs","sync.rs","timer.rs"]],\
"oak_crypto":["",[["hpke",[],["aead.rs","mod.rs"]]],["encryptor.rs","lib.rs","signer.rs","util.rs"]],\
"oak_dice":["",[],["cert.rs","evidence.rs","lib.rs","utils.rs"]],\
"oak_docker_linux_init":["",[],["init.rs","main.rs"]],\
"oak_echo_linux_init":["",[],["init.rs","main.rs"]],\
"oak_echo_service":["",[],["lib.rs"]],\
"oak_enclave_runtime_support":["",[],["heap.rs","lib.rs","libm.rs"]],\
"oak_functions_abi":["",[],["lib.rs"]],\
"oak_functions_client":["",[],["lib.rs"]],\
"oak_functions_containers_app":["",[],["lib.rs","orchestrator_client.rs"]],\
"oak_functions_containers_launcher":["",[],["lib.rs","lookup.rs","server.rs"]],\
"oak_functions_launcher":["",[],["lib.rs","lookup.rs","server.rs"]],\
"oak_functions_load_test":["",[],["main.rs"]],\
"oak_functions_sdk":["",[],["lib.rs"]],\
"oak_functions_sdk_abi_test_get_storage_item":["",[],["lib.rs"]],\
"oak_functions_sdk_abi_test_invoke_testing":["",[],["lib.rs"]],\
"oak_functions_service":["",[["wasm",[],["api.rs","mod.rs"]]],["instance.rs","lib.rs","logger.rs","lookup.rs"]],\
"oak_functions_test_utils":["",[],["lib.rs"]],\
"oak_grpc_utils":["",[],["lib.rs"]],\
"oak_hello_world_linux_init":["",[],["init.rs","main.rs"]],\
"oak_launcher_utils":["",[],["channel.rs","launcher.rs","lib.rs"]],\
"oak_linux_boot_params":["",[],["lib.rs"]],\
"oak_remote_attestation":["",[],["attester.rs","dice.rs","handler.rs","lib.rs"]],\
"oak_remote_attestation_verification":["",[],["lib.rs","rekor.rs","verifier.rs"]],\
"oak_restricted_kernel":["",[["boot",[],["mod.rs"]],["mm",[],["bitmap_frame_allocator.rs","encrypted_mapper.rs","frame_allocator.rs","mod.rs","page_tables.rs","virtual_address_allocator.rs"]],["syscall",[],["channel.rs","dice_data.rs","fd.rs","key.rs","mmap.rs","mod.rs","process.rs","stdio.rs"]]],["acpi.rs","args.rs","avx.rs","descriptors.rs","dice_attestation.rs","elf.rs","ghcb.rs","interrupts.rs","lib.rs","libm.rs","logging.rs","memory.rs","payload.rs","shutdown.rs","snp.rs","snp_guest.rs","virtio.rs"]],\
"oak_restricted_kernel_api":["",[],["channel.rs","lib.rs","logging.rs","raw_syscall.rs","syscall.rs"]],\
"oak_restricted_kernel_interface":["",[],["errno.rs","lib.rs","syscalls.rs"]],\
"oak_sev_guest":["",[],["ap_jump_table.rs","cpuid.rs","crypto.rs","ghcb.rs","guest.rs","instructions.rs","interrupts.rs","io.rs","lib.rs","msr.rs","secrets.rs","vmsa.rs"]],\
"oak_simple_io":["",[],["lib.rs"]],\
"oak_stage0":["",[],["acpi.rs","acpi_tables.rs","allocator.rs","apic.rs","cmos.rs","dice_attestation.rs","fw_cfg.rs","initramfs.rs","kernel.rs","lib.rs","logging.rs","msr.rs","paging.rs","pic.rs","sev.rs","smp.rs","zero_page.rs"]],\
"oak_tdx_guest":["",[],["lib.rs","tdcall.rs","vmcall.rs"]],\
"oak_transparency_claims":["",[],["claims.rs","intoto.rs","lib.rs"]],\
"oak_virtio":["",[["console",[],["mod.rs"]],["queue",[],["mod.rs","virtq.rs"]],["vsock",[["socket",[],["mod.rs"]]],["mod.rs","packet.rs"]]],["lib.rs"]],\
"quirk_echo_launcher":["",[],["lib.rs"]],\
"quirk_echo_service":["",[],["lib.rs"]],\
"sev_serial":["",[],["lib.rs"]],\
"snp_measurement":["",[],["elf.rs","main.rs","page.rs","stage0.rs","vmsa.rs"]],\
"weather_lookup":["",[],["lib.rs"]],\
"xtask":["",[],["check_build_licenses.rs","check_license.rs","check_todo.rs","containers.rs","diffs.rs","examples.rs","files.rs","internal.rs","launcher.rs","lib.rs","testing.rs"]]\
}');
createSrcSidebar();
