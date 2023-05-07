<script lang="ts">
	// import { page } from '$app/stores';
	// import Tabs from '../../../components/Tabs.svelte';
	// import Categories from './ResourceComponents/Categories/+page.svelte';
	// import SubCategories from './ResourceComponents/SubCategories/+page.svelte';

	/*
        add components for:
            - Writings
                - Book
                - Journal
            - Web
                - article
                - interactive
                - journal
                - study
            - Images
                - Infographic
            - Audio
                - Podcast
            - Video
                - YouTube
            - Social Media Content
                - reddit
                - twitter

        Then show each content as
            - title
            - id
            - url
            - description
            - media type
            - media subtype
            - author
            - channel/site name
    */

	export let props: any;

	console.log(props);
</script>

<div class="mb-5">
	<!-- list of filtered drop-down selectors -->
	<ul class="grid grid-cols-4">
		<li class="list-none">
			<!-- With checkboxes -->
			<details role="list">
				<summary aria-haspopup="listbox">Categories</summary>
				<ul role="listbox">
					<li>
						<label>
							<input type="checkbox" />
							Banana
						</label>
					</li>
					<li>
						<label>
							<input type="checkbox" />
							Watermelon
						</label>
					</li>
					<li>
						<label>
							<input type="checkbox" />
							Apple
						</label>
					</li>
				</ul>
			</details>
		</li>
		<li class="list-none">
			<!-- With checkboxes -->
			<details role="list">
				<summary aria-haspopup="listbox">Sub-categories</summary>
				<ul role="listbox">
					<li>
						<label>
							<input type="checkbox" />
							Banana
						</label>
					</li>
					<li>
						<label>
							<input type="checkbox" />
							Watermelon
						</label>
					</li>
					<li>
						<label>
							<input type="checkbox" />
							Apple
						</label>
					</li>
				</ul>
			</details>
		</li>
		<li class="list-none">
			<!-- With checkboxes -->
			<details role="list">
				<summary aria-haspopup="listbox">Topics</summary>
				<ul role="listbox">
					<li>
						<label>
							<input type="checkbox" />
							Banana
						</label>
					</li>
					<li>
						<label>
							<input type="checkbox" />
							Watermelon
						</label>
					</li>
					<li>
						<label>
							<input type="checkbox" />
							Apple
						</label>
					</li>
				</ul>
			</details>
		</li>
		<li class="list-none">
			<!-- With checkboxes -->
			<details role="list">
				<summary aria-haspopup="listbox">Authors</summary>
				<ul role="listbox">
					<li>
						<label>
							<input type="checkbox" />
							Banana
						</label>
					</li>
					<li>
						<label>
							<input type="checkbox" />
							Watermelon
						</label>
					</li>
					<li>
						<label>
							<input type="checkbox" />
							Apple
						</label>
					</li>
				</ul>
			</details>
		</li>
	</ul>
	<!-- List of results -->
	<ul>
		<li>
			<div class="flex rounded-3xl p-5 bg-gray-800 mb-6">
				<div class="pr-3">
					<img
						src="https://t3.ftcdn.net/jpg/02/48/42/64/360_F_248426448_NVKLywWqArG2ADUxDq6QprtIzsF82dMF.jpg"
						alt=""
						class="w-30 pr-5 rounded-xl"
					/>
					<p>Duration:</p>
				</div>
				<div class="">
					<h3 class="text-2xl">Title goes Here</h3>
					<p class="text-xs">Author goes here</p>
					<p class="my-4">
						Description here Lorem ipsum dolor sit amet, consectetur adipisicing elit. Libero,
						facere?
					</p>
					<ul class="flex justify-start flex-row flex-wrap">
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
					</ul>
				</div>
			</div>
		</li>
		<li>
			<div class="flex rounded-3xl p-5 bg-gray-800 mb-6">
				<div class="pr-3">
					<img
						src="https://t3.ftcdn.net/jpg/02/48/42/64/360_F_248426448_NVKLywWqArG2ADUxDq6QprtIzsF82dMF.jpg"
						alt=""
						class="w-30 pr-5 rounded-xl"
					/>
					<p>Duration:</p>
				</div>
				<div class="">
					<h3 class="text-2xl">Title goes Here</h3>
					<p class="text-xs">Author goes here</p>
					<p class="my-4">
						Description here Lorem ipsum dolor sit amet, consectetur adipisicing elit. Libero,
						facere?
					</p>
					<ul class="flex justify-start flex-row flex-wrap">
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
					</ul>
				</div>
			</div>
		</li>
		<li>
			<div class="flex rounded-3xl p-5 bg-gray-800 mb-6">
				<div class="pr-3">
					<img
						src="https://t3.ftcdn.net/jpg/02/48/42/64/360_F_248426448_NVKLywWqArG2ADUxDq6QprtIzsF82dMF.jpg"
						alt=""
						class="w-30 pr-5 rounded-xl"
					/>
					<p>Duration:</p>
				</div>
				<div class="">
					<h3 class="text-2xl">Title goes Here</h3>
					<p class="text-xs">Author goes here</p>
					<p class="my-4">
						Description here Lorem ipsum dolor sit amet, consectetur adipisicing elit. Libero,
						facere?
					</p>
					<ul class="flex justify-start flex-row flex-wrap">
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
					</ul>
				</div>
			</div>
		</li>
		<li>
			<div class="flex rounded-3xl p-5 bg-gray-800 mb-6">
				<div class="pr-3">
					<img
						src="https://t3.ftcdn.net/jpg/02/48/42/64/360_F_248426448_NVKLywWqArG2ADUxDq6QprtIzsF82dMF.jpg"
						alt=""
						class="w-30 pr-5 rounded-xl"
					/>
					<p>Duration:</p>
				</div>
				<div class="">
					<h3 class="text-2xl">Title goes Here</h3>
					<p class="text-xs">Author goes here</p>
					<p class="my-4">
						Description here Lorem ipsum dolor sit amet, consectetur adipisicing elit. Libero,
						facere?
					</p>
					<ul class="flex justify-start flex-row flex-wrap">
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 bg-red-900">Topic</li>
					</ul>
				</div>
			</div>
		</li>
	</ul>
</div>
