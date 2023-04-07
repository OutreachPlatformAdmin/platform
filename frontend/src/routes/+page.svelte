<header>
	<!-- Banner image -->
	<figure class="relative">
		<img
			class="object-cover w-full h-[400px] rounded-xl"
			src="/images/banner-image.jpg"
			alt="banner of a hand and globe"
		/>
		<figcaption class="absolute right-0 bottom-0 z-10 opacity-75 mr-5">
			Photo by <a
				href="https://unsplash.com/@weirick?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
				class="text-cyan-600">Jake Weirick</a
			>
			on
			<a
				href="https://unsplash.com/photos/C47_TXZZwIM?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
				class="text-cyan-500">Unsplash</a
			>
		</figcaption>
	</figure>

	<!-- Welcome text and intro wealth inequality video -->
	<section class="">
		<h1 class="w-full mt-5 pt-3 pb-5 text-5xl border-t-4 border-b-4 border-slate-500 text-center">
			Welcome
		</h1>
		<div>
			<p class="my-5 indent-10">
				Welcome to a platform for learning how the world works, what can be done about it, and how
				you can get involved. Check out the intro video to learn why we bothered creating this
				space.
			</p>
			<p class="indent-10">
				If you're not sure where to start, check out the guided learning path for a structured
				school-like learning experience, or ask some common questions in the questions section.
				Either way, browse around and learn how we can change the world for the better.
			</p>
		</div>
		<!-- svelte-ignore a11y-media-has-caption -->
		<div class="w-full flex justify-center my-10">
			<iframe
				class="w-full aspect-video max-w-4xl rounded-lg"
				src="https://www.youtube.com/embed/uWSxzjyMNpU"
				title="YouTube video player"
				frameborder="0"
				allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
				allowfullscreen
			/>
		</div>
	</section>

	<!-- Interactive wealth inequality -->
	<section>
		<hgroup>
			<h2 class="text-3xl mb-4">
				Would taxing the richest handful of billionaires solve anything?
			</h2>
			<h3 class="indent-10">
				It's almost impossible to visualize a billion dollars, let along a several trillion dollars,
				the net wealth of around a dozen or so billionaires. Scroll to the right in the below
				graphic to better understand just how wrong our system is that it allows and encourages one
				person to keep so much of the profits while so many of the worlds problems could be solved
				by a fraction of it.
				<br />
				<br />
				Just keep scrolling, trust us! Take about 5 minutes to keep scrolling.
			</h3>
		</hgroup>
		<div class="relative">
			<figure>
				<iframe
					src="https://mkorostoff.github.io/1-pixel-wealth/"
					height="600"
					class="w-full my-10 rounded-lg"
					title="Wealth visualized"
				/>
				<figcaption class="absolute right-5 bottom-10">
					<a href="https://mkorostoff.github.io/1-pixel-wealth/"
						>https://mkorostoff.github.io/1-pixel-wealth</a
					>
				</figcaption>
			</figure>
		</div>
	</section>

	<!-- Call to action to introductory sections -->
	<section class="flex flex-col">
		<div class="w-10/12 mb-10  border-yellow-600 border-4 p-5 rounded-3xl">
			<h2
				class="pt-3 pb-5 text-5xl border-t-4 border-b-4 border-slate-500 text-center text-yellow-300"
			>
				New? Start here!
			</h2>
			<p class="mt-5 indent-10">
				If you're new to economics, socialism, capitalism, politics, or any other boring but import
				topic, start here. This will guide you through slowly from feeling powerless at work to
				understanding how to take the power back.
			</p>
			<a href="/starthere" class="my-5 w-full" role="button">Start here!</a>
		</div>
		<div class="w-10/12 self-end border-orange-600 border-4 p-5 rounded-3xl">
			<h2
				class="pt-3 pb-5 text-5xl border-t-4 border-b-4 border-slate-500 text-center text-orange-500"
			>
				Got questions?
			</h2>
			<p class="mt-5 indent-10">
				Click the button to access the entire knowledge database and get answers to many common
				questions
			</p>
			<a href="/questions" class="my-5 w-full" role="button">Search for answers</a>
		</div>
	</section>

	<!-- Browse Categories -->
	<section>
		<hgroup>
			<h2 class="pt-3 pb-5 text-5xl border-t-4 border-b-4 border-slate-500 text-center">
				Browse Categories
			</h2>
			<h3 class="px-10 mt-3">
				Browse any content you'd like to educate yourself or learn how to get involved. We recommend
				starting with myths & FAQs or with courses.
			</h3>
		</hgroup>

		<!-- Grid of categories -->
		<div class="md:grid md:grid-cols-2 md:gap-16">
			<!-- Courses -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-education.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@raddfilms?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Redd F</a
						>
						on
						<a
							href="https://unsplash.com/photos/9o8YdYGTT64?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Courses</h4>
				<p class="text-center">
					Browse courses to learn new topics or help guide your learning journey.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Questions & FAQ -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-questions.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@towfiqu999999?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Towfiqu barbhuiya</a
						>
						on
						<a
							href="https://unsplash.com/photos/9o8YdYGTT64?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Myths & FAQs</h4>
				<p class="text-center">
					Search through all of the myths and frequently asked questions about socialism,
					capitalism, and all the other isms.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Digital Tools -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-digital-tools.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@marvelous?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Marvin Meyer</a
						>
						on
						<a
							href="https://unsplash.com/photos/SYTO3xs06fU?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Digital Tools</h4>
				<p class="text-center">
					Looking for help forming a union? Want help fundraising for a campaign? Want to develop
					marketing material for your campaign? Check out our library of digital tools to help with
					organizing, campaigning, and activism.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Research & Resources -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-research.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@homajob?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Scott Graham</a
						>
						on
						<a
							href="https://unsplash.com/photos/5fNmWej4tAA?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Research & Resources</h4>
				<p class="text-center">
					Looking for help forming a union? Want help fundraising for a campaign? Want to develop
					marketing material for your campaign?
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Unions -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-unions.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@mannyb?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Manny Becerra</a
						>
						on
						<a
							href="https://unsplash.com/photos/GTPK-Wt1qoY?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Unions</h4>
				<p class="text-center">
					Learn why unions are so important, what your rights are, and how to use unions to improve
					your life.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Policies -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-policies.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@glenncarstenspeters?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Glenn Carstens-Peters</a
						>
						on
						<a
							href="https://unsplash.com/photos/RLw-UC03Gwc?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Policies</h4>
				<p class="text-center">
					Learn what to fight for and how to determine a good political leader from a bad one.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Get Involved -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-activism.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@vidarnm?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Vidar Nordli-Mathisen</a
						>
						on
						<a
							href="https://unsplash.com/photos/cR03g2ud84g?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Get Involved</h4>
				<p class="text-center">
					Learn how you can run for a political position, volunteer, fund raise, organize, and
					otherwise contribute to the movement.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Readings $ Theory -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-readings.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@vonshnauzer?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Egor Myznik</a
						>
						on
						<a
							href="https://unsplash.com/photos/1dx1gvuT45o?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Readings & Theory</h4>
				<p class="text-center">
					Dig deeper into theory and discovery historic events, impactful authors, and world-view
					altering perspectives.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Readings & Theory -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-entertainment.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/@cdx2?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">C D-X</a
						>
						on
						<a
							href="https://unsplash.com/images/things/music?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Entertainment</h4>
				<p class="text-center">
					Find music, movie, website, video, podcast, game, and more suggestions related to
					progressivism and the socialist movement.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>

			<!-- Health & Lifestyle -->
			<div class="mb-10">
				<figure class="relative">
					<img
						class="object-cover w-full h-[300px] rounded-xl"
						src="/images/category-health.jpg"
						alt="education category"
					/>
					<figcaption
						class="absolute right-0 bottom-0 z-10 text-sky-100 bg-slate-900 px-2 rounded-tl-full pl-5 bg-opacity-90 text-xs"
					>
						Photo by <a
							href="https://unsplash.com/es/@esdesignisms?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-sky-300 underline">Emma Simpson</a
						>
						on
						<a
							href="https://unsplash.com/images/things/health?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
							class="text-lime-300 underline">Unsplash</a
						>
					</figcaption>
				</figure>
				<h4 class="text-center uppercase mt-5 text-5xl mb-10">Health & Lifestyle</h4>
				<p class="text-center">
					Discovery bits of information and tips on lifestyle management and healthy living.
				</p>
				<a href="#" class="w-full bg-neutral-900 border-neutral-500 mt-5" role="button">Explore</a>
			</div>
		</div>
	</section>

	<!-- Feedback and donation -->
	<section>
		<div class="flex justify-around mb-10">
			<p class="text-xl">Want to contribute to the website? Get in touch here.</p>
			<a href="#" role="button" class="bg-sky-600 border-sky-950">Get In Touch</a>
		</div>
		<div class="flex justify-around">
			<p class="text-xl">Want to donate to the website? Use the button to help out</p>
			<a href="#" role="button" class="bg-lime-600 border-green-950">Donate</a>
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
			<li class="list-none"><a href="/start-here">New? Start here</a></li>
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
