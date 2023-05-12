<script lang="ts">
	import '../app.css';
	// import '../app.postcss';
	import { Menu } from 'lucide-svelte';

	// Your selected Skeleton theme:
	// import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	// import '@skeletonlabs/skeleton/themes/theme-crimson.css';

	// This contains the bulk of Skeletons required styles:
	// NOTE: this will be renamed skeleton.css in the v2.x release.
	// import '@skeletonlabs/skeleton/styles/skeleton.css';

	// Finally, your application's global stylesheet (sometimes labeled 'app.css')
	// import '../app.postcss';
</script>

<nav class=" bg-slate-800 px-5 rounded-md lg:w-8/12 flex flex-row mx-auto">
	<ul>
		<li>
			<a href="/" class="secondary"><Menu /></a>
		</li>
	</ul>
	<ul>
		<li class="text-3xl"><strong>Platform</strong></li>
	</ul>
	<ul>
		<li>
			<a
				href="/startHere"
				role="button"
				class="bg-lime-800 secondary text-white font-bold border-none">Start Here</a
			>
		</li>
		<li>
			<a href="/terms" role="button" class="bg-lime-800 secondary text-white font-bold border-none"
				>Terms</a
			>
		</li>
		<li>
			<a
				href="/CRM/CRMTerm"
				role="button"
				class="bg-lime-800 secondary text-white font-bold border-none">CRM</a
			>
		</li>
	</ul>
</nav>

<slot />
