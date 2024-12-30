<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount, onDestroy } from 'svelte';

	type SystemDetails = {
		hostname: string;
		platform: string;
		distribution: string;
		cpu_model: string;
		cpu_cores: string;
		cpu_speed: string;
	};

	let syncTime = 5;
	let dashboardInterval: NodeJS.Timeout, memoryInterval: NodeJS.Timeout;

	let sysDetails: SystemDetails = {
		hostname: '',
		platform: '',
		distribution: '',
		cpu_model: '',
		cpu_cores: '',
		cpu_speed: ''
	};

	let cpu = '0',
		used_memory = '0',
		total_memory = '0';

	$: memory_percent = (parseInt(used_memory) / parseInt(total_memory)) * 100;
	async function getDashBoardDetails() {
		cpu = await invoke('get_cpu_usage');
	}

	async function getMemoryDetails() {
		let memory: { used: string; total: string } = await invoke('get_memory_usage');
		used_memory = memory.used;
		total_memory = memory.total;
	}

	onMount(async () => {
		dashboardInterval = setInterval(getDashBoardDetails, syncTime * 1000);
		memoryInterval = setInterval(getMemoryDetails, syncTime * 1000);

		sysDetails = await invoke('get_system_details');
	});

	onDestroy(() => {
		clearInterval(dashboardInterval);
		clearInterval(memoryInterval);
	});
</script>

<main>
	<div class="flex flex-col gap-5">
		<div class="flex gap-10">
			<div
				class="bg-surface-600 p-8 flex flex-col justify-center items-center gap-5 rounded-lg shadow-xl"
			>
				<h1>CPU</h1>
				<ProgressRadial
					value={parseInt(cpu)}
					stroke={80}
					meter="stroke-secondary-500"
					track="stroke-secondary-500/30"
					strokeLinecap="butt">{parseInt(cpu)}%</ProgressRadial
				>
			</div>
			<div
				class="bg-surface-600 p-8 flex flex-col justify-center items-center gap-5 rounded-lg shadow-xl"
			>
				<h1>MEMORY</h1>
				<ProgressRadial
					value={parseInt(memory_percent.toString())}
					stroke={80}
					meter="stroke-success-500"
					track="stroke-success-500/30"
					strokeLinecap="butt">{parseInt(used_memory)} GiB</ProgressRadial
				>
			</div>
			<div
				class="bg-surface-600 p-8 flex flex-col justify-center items-center gap-5 rounded-lg shadow-xl"
			>
				<h1>DISK</h1>
				<ProgressRadial
					value={parseInt(memory_percent.toString())}
					stroke={80}
					meter="stroke-warning-500"
					track="stroke-warning-500/30"
					strokeLinecap="butt">{parseInt(used_memory)}</ProgressRadial
				>
			</div>
		</div>
		<div class="flex flex-col">
			<h1 class="text-xl py-4">SYSTEM INFO</h1>
			<p class="text-lg text-surface-400">Hostname: {sysDetails.hostname}</p>
			<p class="text-lg text-surface-400">Platform: {sysDetails.platform}</p>
			<p class="text-lg text-surface-400">Distribution: {sysDetails.distribution}</p>
			<p></p>
			<p class="text-lg text-surface-400">Cpu Model: {sysDetails.cpu_model}</p>
			<p class="text-lg text-surface-400">Cpu Cores: {sysDetails.cpu_cores}</p>
			<p class="text-lg text-surface-400">Cpu Speed: {sysDetails.cpu_speed}</p>
		</div>
	</div>
</main>
