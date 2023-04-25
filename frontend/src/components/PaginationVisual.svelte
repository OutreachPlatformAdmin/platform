<!-- Pagination.svelte -->
<script lang="ts">
	export let currentPage = 1;
	export let totalPages = 1;
	export let onPageChange: (pageNumber: number) => void;

	function changePage(page: number) {
		if (page === currentPage) return;
		if (page < 1 || page > totalPages) return;
		currentPage = page;
		onPageChange(currentPage);
	}

	function previousPage() {
		changePage(currentPage - 1);
	}

	function nextPage() {
		changePage(currentPage + 1);
	}

	function range(start: number, end: number) {
		return Array.from({ length: end - start + 1 }, (_, i) => start + i);
	}
</script>

<div class="pagination">
	<button
		class="pagination-button"
		class:first-page={currentPage === 1}
		disabled={currentPage === 1}
		on:click={previousPage}
	>
		<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none">
			<path d="M19 12L5 12" stroke="#333" stroke-width="2" stroke-linecap="round" />
			<path d="M12 19L5 12L12 5" stroke="#333" stroke-width="2" stroke-linecap="round" />
		</svg>
	</button>

	{#each range(1, totalPages) as page}
		<button
			class="pagination-button"
			class:selected={page === currentPage}
			on:click={() => changePage(page)}
		>
			{page}
		</button>
	{/each}

	<button
		class="pagination-button"
		class:last-page={currentPage === totalPages}
		disabled={currentPage === totalPages}
		on:click={nextPage}
	>
		<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none">
			<path d="M5 12L19 12" stroke="#333" stroke-width="2" stroke-linecap="round" />
			<path d="M12 19L19 12L12 5" stroke="#333" stroke-width="2" stroke-linecap="round" />
		</svg>
	</button>
</div>

<style>
	.pagination {
		display: flex;
		justify-content: center;
		align-items: center;
		margin: 1rem 0;
	}

	.pagination-button {
		border: none;
		background-color: #fff;
		color: #333;
		font-size: 1rem;
		margin: 0 0.5rem;
		padding: 0.5rem;
		cursor: pointer;
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 50%;
		width: 2rem;
		height: 2rem;
		transition: all 0.2s;
	}

	.pagination-button:hover {
		background-color: #f0f0f0;
	}

	.pagination-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.pagination-button:first-page {
		margin-right: 1rem;
	}

	.pagination-button:last-page {
		margin-left: 1rem;
	}

	.pagination-button:first-page svg {
		transform: rotate(180deg);
	}

	.pagination-button svg {
		width: 1rem;
		height: 1rem;
	}
</style>
