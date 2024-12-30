<script lang="ts">
	import DashBoard from "$lib/components/DashBoard.svelte";
	import SideBar from "$lib/components/SideBar.svelte";

	import { type ComponentType } from "svelte";
  let sidebarOpen = false;

	let comp: ComponentType = DashBoard;
	let title: string = "dashboard";
	function selectedComp(component : ComponentType, compTitle: string) {
		comp = component;
		title = compTitle;
	}

</script>

<div class="min-h-screen bg-surface-700">
  <SideBar isOpen={sidebarOpen} {selectedComp}/>
  
  <main class="transition-all duration-300 ease-in-out" class:ml-64={sidebarOpen} class:ml-16={!sidebarOpen}>
    <div class="p-4 flex justify-center items-center flex-col">
      <h1 class="text-2xl font-bold mb-4 capitalize">{title}</h1>
      <svelte:component this={comp} />
    </div>
  </main>
</div>

<style lang="css">
  :global(body) {
    margin: 0;
    padding: 0;
  }
</style>