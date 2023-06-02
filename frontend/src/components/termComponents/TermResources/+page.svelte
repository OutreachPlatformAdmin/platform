<script lang="ts">
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';
	import FaArrowDown from 'svelte-icons/fa/FaArrowDown.svelte';
	export let props: any;

	// console.log(props);
</script>

<div class="mb-5">
	<!-- list of filtered drop-down selectors -->
	<h2 class="mt-10 text-4xl">Filters</h2>
	<ul class="grid grid-cols-4 gap-4 mb-10 mt-4">
		<li class="list-none">
			<!-- With checkboxes -->
			<Accordion class="variant-filled-secondary rounded rounded-xl border border-lime-900">
				<AccordionItem>
					<!-- <svelte:fragment slot="lead">Category</svelte:fragment> -->
					<svelte:fragment slot="summary">Categories</svelte:fragment>
					<svelte:fragment slot="content"
						><ul role="listbox">
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
						</ul></svelte:fragment
					>
				</AccordionItem>
				<!-- ... -->
			</Accordion>
		</li>
		<li>
			<Accordion class="variant-filled-secondary rounded rounded-xl">
				<AccordionItem>
					<!-- <svelte:fragment slot="lead">Authors</svelte:fragment> -->
					<svelte:fragment slot="summary">Authors</svelte:fragment>
					<svelte:fragment slot="content"
						><ul role="listbox">
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
						</ul></svelte:fragment
					>
				</AccordionItem>
				<!-- ... -->
			</Accordion>
		</li>
		<li>
			<Accordion class="variant-filled-secondary rounded rounded-xl">
				<AccordionItem>
					<!-- <svelte:fragment slot="lead">Categories</svelte:fragment> -->
					<svelte:fragment slot="summary">Type</svelte:fragment>
					<svelte:fragment slot="content"
						><ul role="listbox">
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
						</ul></svelte:fragment
					>
				</AccordionItem>
				<!-- ... -->
			</Accordion>
		</li>
		<li>
			<Accordion class="variant-filled-secondary rounded rounded-xl">
				<AccordionItem>
					<!-- <svelte:fragment slot="lead">Categories</svelte:fragment> -->
					<svelte:fragment slot="summary">Topic</svelte:fragment>
					<svelte:fragment slot="content"
						><ul role="listbox">
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
						</ul></svelte:fragment
					>
				</AccordionItem>
				<!-- ... -->
			</Accordion>
		</li>
	</ul>
	<!-- List of results -->
	<ul class="md:grid md:grid-cols-2 ">
		<li class="list-none mx-5">
			<div class="sm:flex rounded-3xl p-5 mb-6 variant-filled">
				<div class="pr-3 max-w-sm mx-auto">
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
					<ul
						class="flex flex-wrap flex-row justify-around text-xs overflow-auto h-14 variant-glass-secondary rounded-xl"
					>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
					</ul>
				</div>
			</div>
		</li>
		<li class="list-none mx-5">
			<div class="sm:flex rounded-3xl p-5 mb-6 variant-filled-primary">
				<div class="pr-3 max-w-sm mx-auto">
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
					<ul
						class="flex flex-wrap flex-row justify-around text-xs overflow-auto h-14 variant-glass-secondary rounded-xl"
					>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
					</ul>
				</div>
			</div>
		</li>
		<li class="list-none mx-5">
			<div class="sm:flex rounded-3xl p-5 mb-6 variant-filled-primary">
				<div class="pr-3 max-w-sm mx-auto">
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
					<ul
						class="flex flex-wrap flex-row justify-around text-xs overflow-auto h-14 variant-glass-secondary rounded-xl"
					>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
					</ul>
				</div>
			</div>
		</li>
		<li class="list-none mx-5">
			<div class="sm:flex rounded-3xl p-5 mb-6 variant-filled-primary">
				<div class="pr-3 max-w-sm mx-auto">
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
					<ul
						class="flex flex-wrap flex-row justify-around text-xs overflow-auto h-14 variant-glass-secondary rounded-xl"
					>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
					</ul>
				</div>
			</div>
		</li>
		<li class="list-none mx-5">
			<div class="sm:flex rounded-3xl p-5 mb-6 variant-filled-primary">
				<div class="pr-3 max-w-sm mx-auto">
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
					<ul
						class="flex flex-wrap flex-row justify-around text-xs overflow-auto h-14 variant-glass-secondary rounded-xl"
					>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
					</ul>
				</div>
			</div>
		</li>
		<li class="list-none mx-5">
			<div class="sm:flex rounded-3xl p-5 mb-6 variant-filled-primary">
				<div class="pr-3 max-w-sm mx-auto">
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
					<ul
						class="flex flex-wrap flex-row justify-around text-xs overflow-auto h-14 variant-glass-secondary rounded-xl"
					>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
						<li class="list-none rounded-3xl py-1 px-3 mr-2 variant-filled-secondary my-2">
							Topic
						</li>
					</ul>
				</div>
			</div>
		</li>
	</ul>
</div>
