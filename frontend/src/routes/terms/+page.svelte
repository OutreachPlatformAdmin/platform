<script lang="ts">
	let terms = ['capitalism', 'socialism', 'union', 'Strike action'];

	const getTerms = async () => {
		const data = await fetch(`http://localhost:3000/terms`);
		let result = await data.json();
	};
</script>

<button on:click={getTerms}>Get terms</button>

<nav class="list-nav my-10 grid grid-cols-2">
	<ul>
		{#each terms as term}
			<li>
				<a href="/terms/{term}" class="flex-auto">{term}</a>
			</li>
		{/each}
	</ul>
</nav>
