<script lang="ts">
	// import StartHere from '../routes/startHere/+page.svelte';
	// import InteractiveFAQ from '../homepageComponents/InteractiveFAQ/+page.svelte';
	// import BriefIntro from '../homepageComponents/BriefIntro/+page.svelte';
	// import InteractiveWealth from '../homepageComponents/InteractiveWealth/+page.svelte';
	// const webpages = [
	// 	{ name: 'Brief Intro', component: BriefIntro },
	// 	{ name: 'Interactive FAQ', component: InteractiveFAQ },
	// 	{ name: 'Interactive Wealth', component: InteractiveWealth },
	// 	{ name: 'About', component: StartHere }
	// ];

	// import { page } from '$app/stores';

	export let webpages: any[];
	console.log(webpages, 'here');
	// Loads an object in webpages array
	let selectedPage = webpages[0];
	// $: console.dir(selectedPage);

	// Have to use obj as arg. so value can be a class
	const loadPage = (obj: any) => (selectedPage = obj);
</script>

<div>
	<div class="pb-20">
		{#each webpages as webpageObj}
			<button class="tablink" title={webpageObj.name} on:click={() => loadPage(webpageObj)}
				>{webpageObj.name}</button
			>
		{/each}
	</div>

	<!-- Loaded component/webpage -->
	<svelte:component this={selectedPage.component} />
</div>

<!-- 
Tabs component
     input: array of names and page links as "tabs" variable
	 let selectedPage = tabs[0] by default
	update active page using const loadPage = (obj: any) => (selectedPage = obj);

    show tabs using
	<div>
		{#each webpages as webpageObj}
			<button class="tablink" title={webpageObj.name} on:click={() => loadPage(webpageObj)}>{webpageObj.name}</button>
		{/each}

		show active page using
		<svelte:component this={selectedPage.component} />
	</div>


Term selected
	- fetches term from database
	- term includes: {
		- id
		- term
		- brief description
		- full description
		- wiki url
		- wiki description
		- wiki image url
		- bullet points [point]
		- examples [example]
		- parallells [parallel]
		- resources [sourceID]
	}

	Term page
		load Tabs component
 -->
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
