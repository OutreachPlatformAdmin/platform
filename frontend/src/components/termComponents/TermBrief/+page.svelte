<script lang="ts">
	export let props: any;
	const html = props.brief.replace(/\n/g, '<br><br>');
	console.log(props.bulletPoints);
</script>

<div>
	<div class="p-5 mb-5">
		<h2 class="pb-2 border-b-[1px] mb-5 text-2xl">Brief Description</h2>
		<p>
			{@html html}
		</p>
	</div>
	<div class="p-5 mb-10">
		<h2 class="pb-2 text-2xl border-b-[1px] mb-8">Bullet Points</h2>
		<ul class="grid grid-cols-2 gap-10">
			{#each props.bulletPoints as point, index}
				<li class="border-r-2 pr-2">
					{point}
				</li>
			{/each}
		</ul>
	</div>
</div>

<style>
	p {
		white-space: pre-wrap;
		line-height: 30px;
	}
</style>
