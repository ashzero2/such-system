<script lang="ts">
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	export let isOpen = false;
	import { PanelLeftOpen, PanelLeftClose, Gauge, Network } from 'lucide-svelte';
	import DashBoard from './DashBoard.svelte';
	import Hosts from './Hosts.svelte';
	import type { ComponentType } from 'svelte';

	type MenuItem = {
		label: string;
		icon: ComponentType;
		comp: ComponentType;
	};

	const menuItems: MenuItem[] = [
		{ label: 'Dashboard', icon: Gauge, comp: DashBoard },
		{ label: 'DNS', icon: Network, comp: Hosts }
	];

	export let selectedComp: (comp: ComponentType, title: string) => void;
	let selected = "Dashboard";
	
	function handleSelection(item: MenuItem) {
		selected = item.label;
		selectedComp(item.comp, item.label);
	} 	
</script>

<aside
	class="h-screen fixed left-0 top-0 bg-surface-800 z-40 text-white transition-all duration-300 ease-in-out {isOpen
		? 'w-64'
		: 'w-16'}"
>
	<div class="flex flex-col h-full items-center">
		<!-- Toggle Button -->
		<button
			class="p-4 hover:bg-gray-700 transition-colors duration-200"
			on:click={() => (isOpen = !isOpen)}
		>
			{#if isOpen}
				<PanelLeftClose />
			{:else}
				<PanelLeftOpen />
			{/if}
		</button>

		<!-- Navigation Items -->
		<nav class="flex-1">
			<ul class="py-4">
				{#each menuItems as item}
					<li>
						<button
							on:click|preventDefault={() => handleSelection(item)}
							class="rounded-md flex items-center px-4 py-3 hover:bg-primary-700 {(selected===item.label ? 'bg-primary-700' : 'bg-inherit')} transition-colors duration-200"
						>
							<span class="text-xl">
								<svelte:component this={item.icon} />
							</span>
							{#if isOpen}
								<span class="ml-4">{item.label}</span>
							{/if}
						</button>
					</li>
				{/each}
			</ul>
		</nav>
	</div>
</aside>
