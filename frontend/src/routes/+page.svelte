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
</header>
