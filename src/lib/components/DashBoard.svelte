<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount, onDestroy } from 'svelte';

	let syncTime = 5;
	let dashboardInterval: NodeJS.Timeout, memoryInterval: NodeJS.Timeout;

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
		console.log(parseInt(memory_percent.toString()));
	}

	onMount(async () => {
		dashboardInterval = setInterval(getDashBoardDetails, syncTime * 1000);
		memoryInterval = setInterval(getMemoryDetails, syncTime * 1000);
	});

	onDestroy(() => {
		clearInterval(dashboardInterval);
		clearInterval(memoryInterval);
	});
</script>

<main>
	<div class="flex gap-10">
		<div class="bg-surface-600 p-8 flex flex-col justify-center items-center gap-5 rounded-lg shadow-xl">
			<h1>CPU</h1>
			<ProgressRadial
				value={parseInt(cpu)}
				stroke={80}
				meter="stroke-secondary-500"
				track="stroke-secondary-500/30"
				strokeLinecap="butt">{parseInt(cpu)}%</ProgressRadial
			>
		</div>
		<div class="bg-surface-600 p-8 flex flex-col justify-center items-center gap-5 rounded-lg shadow-xl">
			<h1>MEMORY</h1>
			<ProgressRadial
				value={parseInt(memory_percent.toString())}
				stroke={80}
				meter="stroke-success-500"
				track="stroke-success-500/30"
				strokeLinecap="butt">{parseInt(used_memory)} GiB</ProgressRadial
			>
		</div>
		<div class="bg-surface-600 p-8 flex flex-col justify-center items-center gap-5 rounded-lg shadow-xl">
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
</main>
