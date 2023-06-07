<script lang="ts">
	import '@skeletonlabs/skeleton/themes/theme-crimson.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';

	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';
	import { Drawer, drawerStore } from '@skeletonlabs/skeleton';
	import type { DrawerSettings } from '@skeletonlabs/skeleton';

	import TiThMenu from 'svelte-icons/ti/TiThMenu.svelte';
	import TiHome from 'svelte-icons/ti/TiHome.svelte';

	const drawerSettings: DrawerSettings = {
		id: 'example-3',
		// Provide your property overrides:
		bgDrawer: 'bg-primary-900',
		bgBackdrop: 'bg-gradient-to-tr from-secondary-500/50 via-secondary-800/50 to-tertiary-500/50',
		width: 'w-[280px] md:w-[480px]',
		padding: 'p-4',
		rounded: 'rounded-xl'
	};

	function openRightDrawer() {
		drawerStore.open(drawerSettings);
	}
</script>

<AppShell regionPage="relative" slotHeader="sticky top-0 z-10">
	<svelte:fragment slot="header">
		<AppBar
			gridColumns="grid-cols-3"
			slotDefault="place-self-center"
			slotTrail="place-content-end"
			background="bg-primary-700"
		>
			<svelte:fragment slot="lead"
				><div class="w-10 hover:cursor-pointer variant-filled-primary">
					<a href="/"><TiHome /></a>
				</div></svelte:fragment
			>
			<h1 class="text-3xl text-white">Platform</h1>

			<svelte:fragment slot="trail">
				<LightSwitch />
				<div
					class="w-10 hover:cursor-pointer variant-filled-primary"
					on:click={openRightDrawer}
					on:keypress={openRightDrawer}
				>
					<TiThMenu />
				</div></svelte:fragment
			>
		</AppBar>
	</svelte:fragment>
	<!-- <svelte:fragment slot="sidebarLeft">Sidebar Left</svelte:fragment> -->
	<svelte:fragment slot="sidebarRight"
		><Drawer position="right"
			><nav class="list-nav">
				<ul class="pt-5 px-2">
					<li class="pb-1">
						<a
							href="/startHere"
							class="variant-filled-success hover:variant-filled-secondary"
							on:click={() => drawerStore.close()}
						>
							<span class="badge bg-secondary-500">🚩</span>
							<span class="flex-auto">Start Here</span>
						</a>
					</li>
					<li class="pb-1">
						<a
							href="/terms"
							class="variant-filled-tertiary hover:variant-filled-secondary"
							on:click={() => drawerStore.close()}
						>
							<span class="badge bg-primary-500">📄</span>
							<span class="flex-auto">Terms</span>
						</a>
					</li>
					<li class="pb-1">
						<a
							href="/CRM/CRMTerm"
							class="variant-filled-tertiary hover:variant-filled-secondary"
							on:click={() => drawerStore.close()}
						>
							<span class="badge bg-primary-500">👉</span>
							<span class="flex-auto">CRM Term Entry</span>
						</a>
					</li>
				</ul>
			</nav></Drawer
		></svelte:fragment
	>
	<!-- <svelte:fragment slot="pageHeader">Page Header</svelte:fragment> -->
	<!-- Router Slot -->
	<div class="sm:mx-10 mx-5">
		<slot />
	</div>

	<!-- ---- / ---- -->
	<svelte:fragment slot="pageFooter"
		><!-- Site footer -->
		<footer class="variant-filled-secondary p-5">
			<div class="mb-10">
				<h6 class="text-2xl">About</h6>
				<p>
					Thanks for checking out the site. Browse around areas of the website below and be sure to
					spread the site to education and engage others.
				</p>
			</div>
			<div class="grid grid-cols-2">
				<div class="mb-10">
					<h6 class="text-2xl">Categories</h6>
					<ul class="indent-8">
						<li class="list-none"><a href="/startHere">New? Start here</a></li>
						<li class="list-none"><a href="/questions">Questions?</a></li>
						<li class="list-none"><a href="/courses">Courses</a></li>
						<li class="list-none"><a href="/myths-faq">Myths & FAQs</a></li>
						<li class="list-none"><a href="/digital-tools">Digital Tool</a></li>
						<li class="list-none"><a href="/research">Research & Resources</a></li>
						<li class="list-none"><a href="/unions">Unions</a></li>
						<li class="list-none"><a href="/policies">Policies</a></li>
						<li class="list-none"><a href="/activism">Get Involved</a></li>
						<li class="list-none"><a href="/readings">Readings & Theory</a></li>
						<li class="list-none"><a href="/entertainment">Entertainment</a></li>
						<li class="list-none"><a href="/health">Health & Lifestyle</a></li>
					</ul>
				</div>

				<div class="mb-10">
					<h6 class="text-2xl">Quick Links</h6>
					<ul class="indent-8">
						<li class="list-none"><a href="http://scanfcode.com/about/">About Us</a></li>
						<li class="list-none"><a href="http://scanfcode.com/contact/">Contact Us</a></li>
						<li class="list-none">
							<a href="http://scanfcode.com/contribute-at-scanfcode/">Contribute</a>
						</li>
						<li class="list-none">
							<a href="http://scanfcode.com/privacy-policy/">Privacy Policy</a>
						</li>
						<li class="list-none"><a href="http://scanfcode.com/sitemap/">Sitemap</a></li>
					</ul>
				</div>
			</div>
		</footer></svelte:fragment
	>
	<!-- <svelte:fragment slot="footer" /> -->
</AppShell>
