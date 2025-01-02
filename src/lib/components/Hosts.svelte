<script lang="ts">
	import { show } from '@tauri-apps/api/app';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	type Hosts = [string, string[]];

	let hostEntries: Hosts[] = [];
	let showAdd = true;

	onMount(async () => {
		hostEntries = await invoke('get_hosts');
	});

	function showAddForm() {
		showAdd = !showAdd;
	}
</script>

<main>
	<div class="m-4 flex flex-col gap-5">
		{#if !showAdd}
			<button on:click={showAddForm} type="button" class="btn variant-filled">Add Host</button>
		{/if}
		{#if showAdd}
			<div class="flex gap-3">
				<div class="flex gap-3">
					<input type="text" class="input p-2" placeholder="  Host IP">
					<input type="text" class="input" placeholder="   Hostname">
				</div>
				<div>
					<button class="btn btn-md variant-filled">Add</button>
					<button on:click={showAddForm} class="btn btn-md variant-filled">Close</button>
				</div>
			</div>
		{/if}
		<div class="table-container">
			<table class="table table-hover">
				<thead>
					<tr>
						<th>IP Address</th>
						<th>FQDN</th>
						<th>Aliases</th>
					</tr>
				</thead>
				<tbody>
					{#each hostEntries as host}
						<tr>
							<td>{host[0]}</td>
							<td>{host[1][0]}</td>
							<td>{host[1][1] || '-'}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</main>
