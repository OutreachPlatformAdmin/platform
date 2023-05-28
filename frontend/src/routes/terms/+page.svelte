<script lang="ts">
	let terms = ['capitalism', 'socialism', 'union', 'Strike action'];
</script>

<nav class="list-nav my-10 grid grid-cols-2">
	<ul>
		{#each terms as term}
			<li>
				<a href="/terms/{term}" class="flex-auto">{term}</a>
			</li>
		{/each}
	</ul>
</nav>
