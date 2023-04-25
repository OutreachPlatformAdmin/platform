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
		<!-- <div class=" grid grid-cols-2"> -->
		<!-- <a href="/startHere" class="w-full bg-sky-700" role="button">New? Start here!</a> -->
		<a href="/startHere" class="w-full bg-green-700" role="button">Got Questions?</a>
		<!-- </div> -->
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

<!-- Site footer -->
<footer class="bg-gray-800 p-5">
	<div class="mb-10">
		<h6 class="text-2xl">About</h6>
		<p class="">
			Thanks for checking out the site. Browse around areas of the website below and be sure to
			spread the site to education and engage others.
		</p>
	</div>

	<div class="mb-10">
		<h6 class="text-2xl">Categories</h6>
		<ul class="text-gray-500 indent-8">
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
		<ul class="text-gray-500 indent-8">
			<li class="list-none"><a href="http://scanfcode.com/about/">About Us</a></li>
			<li class="list-none"><a href="http://scanfcode.com/contact/">Contact Us</a></li>
			<li class="list-none">
				<a href="http://scanfcode.com/contribute-at-scanfcode/">Contribute</a>
			</li>
			<li class="list-none"><a href="http://scanfcode.com/privacy-policy/">Privacy Policy</a></li>
			<li class="list-none"><a href="http://scanfcode.com/sitemap/">Sitemap</a></li>
		</ul>
	</div>
</footer>
