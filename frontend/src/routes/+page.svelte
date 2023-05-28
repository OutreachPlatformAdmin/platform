<script lang="ts">
	import Tabs from '../components/Tabs.svelte';
	import Categories from '../components/homepageComponents/Categories/+page.svelte';
	import StartHere from '../routes/startHere/+page.svelte';
	import InteractiveFAQ from '../components/homepageComponents/InteractiveFAQ/+page.svelte';
	import BriefIntro from '../components/homepageComponents/BriefIntro/+page.svelte';
	import InteractiveWealth from '../components/homepageComponents/InteractiveWealth/+page.svelte';

	const webpages = [
		{ name: 'Brief Intro', component: BriefIntro },
		{ name: 'Interactive FAQ', component: InteractiveFAQ },
		{ name: 'Interactive Wealth', component: InteractiveWealth },
		{ name: 'About', component: StartHere }
	];
</script>

<header class="lg:w-8/12 flex flex-col m-auto">
	<!-- Call to action to introductory sections -->
	<section class="">
		<h1
			class="w-full my-5 pt-3 pb-5 lg:text-5xl text-3xl border-t-4 border-b-4 border-slate-500 text-center"
		>
			People over profits
		</h1>
		<a
			href="/startHere"
			type="button"
			class="btn variant-filled mb-5 hover:variant-filled-secondary"
		>
			<span class="badge bg-primary-500">❔</span>
			<span class="flex-auto">Got Querstions? Start Here!</span>
		</a>
	</section>

	<Tabs {webpages} />

	<Categories />

	<!-- Feedback and donation -->
	<section>
		<div class="flex justify-around mb-10">
			<p class="text-xl">Want to contribute to the website? Get in touch here.</p>
			<a href="/" role="button" class="bg-sky-600 border-sky-950">Get In Touch</a>
		</div>
		<div class="flex justify-around">
			<p class="text-xl">Want to donate to the website? Use the button to help out</p>
			<a href="/" role="button" class="bg-lime-600 border-green-950">Donate</a>
		</div>
	</section>
</header>
