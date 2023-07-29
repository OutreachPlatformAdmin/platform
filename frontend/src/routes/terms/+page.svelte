<script lang="ts">
	let terms = ['capitalism', 'socialism', 'union', 'Strike action'];

	const getTerms = async () => {
		const data = await fetch(`http://localhost:3000/terms`);
		let result = await data.json();
	};
</script>

<button class="" on:click={getTerms}>Get terms</button>

	<ul class="grid grid-cols-2 w-screen gap-3 mt-10">
		{#each terms as term}
			<li class="m-5">
				<a href="/terms/{term}" class="p-5 bg-slate-800 rounded-2xl hover:pointer">{term}</a>
			</li>
		{/each}
	</ul>

