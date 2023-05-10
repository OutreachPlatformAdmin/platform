<script lang="ts">
	import PaginationVisual from '../../../../PaginationVisual.svelte';
	export let props: any;
	console.log(props);
	let currentIndex = 1;
	const updateCurrentIndex = (update: number) => {
		currentIndex = update;
	};
</script>

<PaginationVisual
	totalPages={props.length}
	onPageChange={(currentPage) => updateCurrentIndex(currentPage)}
/>
<div class="p-5 mb-5">
	<ul>
		<li class="my-5">
			<h2 class="pb-2 border-b-[1px] mb-5 text-2xl">Resource #{currentIndex}</h2>
			<p>
				{props[currentIndex - 1]}
			</p>
		</li>
	</ul>
</div>
