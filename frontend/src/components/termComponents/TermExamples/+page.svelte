<script lang="ts">
	import { page } from '$app/stores';
	import Tabs from '../../../components/Tabs.svelte';
	import Examples from './ExampleComponents/Examples/+page.svelte';
	import Parallels from './ExampleComponents/Parallells/+page.svelte';
	export let props: any;

	console.log(props);

	const webpages = [
		{ name: 'Examples', component: Examples, props: props.examples },
		{ name: 'Parallels', component: Parallels, props: props.parallels }
	];
</script>

<div class="mb-5">
	<Tabs {webpages} />
</div>
