<script lang="ts">
	import FlipCard from '../../components/FlipCard.svelte';

	let cardStates = false;

	function clicked() {
		cardStates = !cardStates;
	}

	// placeholder data for future common questions
	let cards = [
		{
			id: 1,
			question: 'this is question 1?',
			answer: 'This is the answer for question 1'
		},
		{
			id: 2,
			question: 'this is question 2?',
			answer: 'This is the answer for question 2'
		},
		{
			id: 3,
			question: 'this is question 3?',
			answer: 'This is the answer for question 3'
		}
	];
</script>

<section class="grid lg:grid-cols-3 grid-cols-2 gap-5">
	{#each cards as { question, answer }}
		<FlipCard {question} {answer} />
	{/each}
</section>
