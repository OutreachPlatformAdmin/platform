<script lang="ts">
	export let webpages: any[];
	let selectedPage = webpages[0];

	// Have to use obj as arg. so value can be a class
	const loadPage = (obj: any) => (selectedPage = obj);
</script>

<div class="my-5">
	<!-- <div class="pb-20">
		{#each webpages as webpageObj}
			<button class="tablink" title={webpageObj.name} on:click={() => loadPage(webpageObj)}
				>{webpageObj.name}</button
			>
		{/each}
	</div> -->

	<div class="btn-group variant-filled w-full">
		{#each webpages as webpageObj}
			<button
				title={webpageObj.name}
				on:click={() => loadPage(webpageObj)}
				class="w-full hover:variant-filled-primary">{webpageObj.name}</button
			>
		{/each}
	</div>
	<!-- Loaded component/webpage -->
	<svelte:component this={selectedPage.component} props={selectedPage.props} />
</div>

<style>
	* {
		box-sizing: border-box;
	}
</style>
