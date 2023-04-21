<script lang="ts">
	import { page } from '$app/stores';
	import Tabs from '../../../components/Tabs.svelte';
	import BriefDescription from '../../../components/termComponents/TermBrief/+page.svelte';
	import FullDescription from '../../../components/termComponents/TermFull/+page.svelte';
	import Examples from '../../../components/termComponents/TermExamples/+page.svelte';
	import Resources from '../../../components/termComponents/TermResources/+page.svelte';

	let term = $page.params.term;

	const webpages = [
		{ name: 'Brief', component: BriefDescription },
		{ name: 'Details', component: FullDescription },
		{ name: 'Examples', component: Examples },
		{ name: 'Resources', component: Resources }
	];
</script>

<div class="my-5">
	<Tabs {webpages} />
</div>
