<script lang="ts">
	import '@skeletonlabs/skeleton/themes/theme-crimson.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';

	import { LightSwitch } from '@skeletonlabs/skeleton';
	import { Menu } from 'lucide-svelte';
	import TiThMenu from 'svelte-icons/ti/TiThMenu.svelte';
	import { Drawer, drawerStore } from '@skeletonlabs/skeleton';
	import type { DrawerSettings } from '@skeletonlabs/skeleton';
	import TiHome from 'svelte-icons/ti/TiHome.svelte';
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';

	const drawerSettings: DrawerSettings = {
		id: 'example-3',
		// Provide your property overrides:
		bgDrawer: 'bg-primary-800 text-white',
		bgBackdrop: 'bg-gradient-to-tr from-secondary-800/50 via-primary-800/50 to-secondary-800/50',
		width: 'w-[280px] md:w-[480px]',
		padding: 'p-4',
		rounded: 'rounded-xl'
	};

	function openRightDrawer() {
		drawerStore.open(drawerSettings);
	}
	// drawerStore.open(drawerSettings);
	let valueSingle: string = 'books';
</script>

<Drawer position="right"
	><ListBox>
		<ListBoxItem bind:group={valueSingle} name="medium" value="books">Books</ListBoxItem>
		<ListBoxItem bind:group={valueSingle} name="medium" value="movies">Movies</ListBoxItem>
		<ListBoxItem bind:group={valueSingle} name="medium" value="tv">TV</ListBoxItem>
	</ListBox></Drawer
>
<AppShell regionPage="relative" slotPageHeader="sticky top-0 z-10">
	<svelte:fragment slot="header">
		<AppBar
			gridColumns="grid-cols-3"
			slotDefault="place-self-center"
			slotTrail="place-content-end"
			background="bg-primary-700"
		>
			<svelte:fragment slot="lead"
				><div
					class="w-10 hover:cursor-pointer"
					on:click={openRightDrawer}
					on:keypress={openRightDrawer}
				>
					<TiHome />
				</div></svelte:fragment
			>
			Platform
			<svelte:fragment slot="trail"
				><div
					class="w-10 hover:cursor-pointer"
					on:click={openRightDrawer}
					on:keypress={openRightDrawer}
				>
					<TiThMenu />
				</div></svelte:fragment
			>
		</AppBar>
		<!-- <AppBar class="bg-primary-700"
			><nav class="bg-primary-700 px-5 rounded-md flex flex-row mx-auto py-5">
				<ul>
					<li>
						<a href="/" class="secondary"><Menu /></a>
					</li>
				</ul>
				<ul>
					<li><LightSwitch /></li>
				</ul>
				<ul>
					<li class="text-3xl"><strong>Platform</strong></li>
				</ul>
				<ul class="flex ">
					<li>
						<a href="/startHere" role="button" class="bg-tertiary-100 text-primary-500 rounded p-2"
							>Start Here</a
						>
					</li>
					<li>
						<a
							href="/terms"
							role="button"
							class="bg-lime-800 secondary text-white font-bold border-none">Terms</a
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
			</nav></AppBar
		> -->
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">Sidebar Left</svelte:fragment>
	<svelte:fragment slot="sidebarRight">Sidebar Right</svelte:fragment>
	<svelte:fragment slot="pageHeader">Page Header</svelte:fragment>
	<!-- Router Slot -->
	<slot />
	<!-- ---- / ---- -->
	<svelte:fragment slot="pageFooter">Page Footer</svelte:fragment>
	<svelte:fragment slot="footer">Footer</svelte:fragment>
</AppShell>

<!-- <nav class="bg-primary-700 px-5 rounded-md lg:w-8/12 flex flex-row mx-auto py-5">
	<ul>
		<li>
			<a href="/" class="secondary"><Menu /></a>
		</li>
	</ul>
	<ul>
		<li><LightSwitch /></li>
	</ul>
	<ul>
		<li class="text-3xl"><strong>Platform</strong></li>
	</ul>
	<ul class="flex ">
		<li>
			<a href="/startHere" role="button" class="bg-tertiary-100 text-primary-500 rounded p-2"
				>Start Here</a
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

<slot /> -->
