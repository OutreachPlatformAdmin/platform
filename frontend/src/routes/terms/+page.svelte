<script lang="ts">
	let terms = ['capitalism', 'socialism', 'union', 'Strike action'];
</script>

<ul>
	{#each terms as term}
		<li>
			<a href="/terms/{term}">{term}</a>
		</li>
	{/each}
</ul>
