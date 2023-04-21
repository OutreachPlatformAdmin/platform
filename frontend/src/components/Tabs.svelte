<script lang="ts">
	import StartHere from '../routes/startHere/+page.svelte';
	import InteractiveFAQ from '../homepageComponents/InteractiveFAQ/+page.svelte';
	import BriefIntro from '../homepageComponents/BriefIntro/+page.svelte';
	import InteractiveWealth from '../homepageComponents/InteractiveWealth/+page.svelte';
	const webpages = [
		{ name: 'Brief Intro', component: BriefIntro },
		{ name: 'Interactive FAQ', component: InteractiveFAQ },
		{ name: 'Interactive Wealth', component: InteractiveWealth },
		{ name: 'About', component: StartHere }
	];

	// Loads an object in webpages array
	let selectedPage = webpages[0];
	$: console.dir(selectedPage);

	// Have to use obj as arg. so value can be a class
	const loadPage = (obj: any) => (selectedPage = obj);
</script>

<div>
	{#each webpages as webpageObj}
		<button class="tablink" title={webpageObj.name} on:click={() => loadPage(webpageObj)}
			>{webpageObj.name}</button
		>
	{/each}

	<!-- Loaded component/webpage -->
	<svelte:component this={selectedPage.component} />
</div>

<style>
	* {
		box-sizing: border-box;
	}

	/* Style tab links */
	.tablink {
		background-color: #555;
		color: white;
		float: left;
		border: none;
		outline: none;
		border: 0.5px solid #444;
		border-radius: 0;
		cursor: pointer;
		padding: 14px 16px;
		font-size: 17px;
		width: 25%;
	}

	.tablink:hover {
		background-color: #777;
	}
</style>
