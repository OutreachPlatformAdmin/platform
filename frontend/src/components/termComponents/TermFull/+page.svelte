<script lang="ts">
	export let props: any;
	const html = props.full.replace(/\n/g, '<br><br>');
</script>

<div>
	<div class="p-5 mb-5">
		<h2 class="pb-2 border-b-[1px] mb-5 text-2xl">Full Description</h2>
		<p>
			{@html html}
		</p>
	</div>
</div>

<style>
	p {
		white-space: pre-wrap;
		line-height: 30px;
	}
</style>
