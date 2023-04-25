<script lang="ts">
	import { page } from '$app/stores';
	import Tabs from '../../../components/Tabs.svelte';
	import BriefDescription from '../../../components/termComponents/TermBrief/+page.svelte';
	import FullDescription from '../../../components/termComponents/TermFull/+page.svelte';
	import Examples from '../../../components/termComponents/TermExamples/+page.svelte';
	import Resources from '../../../components/termComponents/TermResources/+page.svelte';

	let term = $page.params.term;

	let propDetails = {
		brief:
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Nihil veniam, labore facilis laboriosam dolor dolorum illum explicabo! Omnis, eveniet eaque!\nLorem ipsum dolor sit, amet consectetur adipisicing elit. Nihil veniam, labore facilis laboriosam dolor dolorum illum explicabo! Omnis, eveniet eaque!',
		bulletPoints: [
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.'
		],
		full: 'Lorem ipsum dolor sit amet consectetur adipisicing elit. Dolor reprehenderit illo dolore mollitia corporis minima iusto molestias vitae. Esse quo rem tempora ad voluptates cumque modi, eum vero nemo minus provident ut dignissimos minima exercitationem ducimus hic eos praesentium earum doloribus. Consequuntur, accusantium commodi facere incidunt eius illo, ratione quaerat facilis eligendi quia recusandae laboriosam sed expedita nulla porro tempora ipsum vero illum aspernatur voluptas odit! Praesentium earum exercitationem laudantium. Exercitationem et quam quia, explicabo laborum dolores sunt magni minus officiis necessitatibus doloribus excepturi aperiam earum esse iure magnam animi, recusandae sit dicta vel. Laboriosam nostrum corrupti sapiente aliquam repellendus.',
		examples: [
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.',
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.',
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.',
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.',
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.'
		],
		parallels: [
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?',
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?',
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?',
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?',
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?'
		]
	};

	const webpages = [
		{ name: 'Brief', component: BriefDescription, props: { ...propDetails } },
		{ name: 'Details', component: FullDescription, props: { ...propDetails } },
		{ name: 'Examples', component: Examples, props: { ...propDetails } },
		{ name: 'Resources', component: Resources, props: { ...propDetails } }
	];
</script>

<div class="my-5">
	<h2>{term}</h2>
	<Tabs {webpages} />
</div>
