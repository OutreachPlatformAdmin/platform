<script lang="ts">
	export let question: string;
	export let answer: string;
</script>

<div class="flip-card bg-transparent w-[300px] h-[300px]">
	<div class="flip-card-inner relative w-full h-full text-center rounded-lg">
		<div class="flip-card-front absolute w-full h-full bg-sky-700 rounded-lg">
			<p class="p-5">
				{question}
			</p>
		</div>
		<div class="flip-card-back w-full h-full absolute bg-lime-700 rounded-lg">
			<p class="p-5">{answer}</p>
		</div>
	</div>
</div>

<style>
	.flip-card {
		perspective: 1000px;
	}

	.flip-card-inner {
		transition: transform 0.6s;
		transform-style: preserve-3d;
		box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
	}

	.flip-card:hover .flip-card-inner {
		transform: rotateY(180deg);
	}

	.flip-card:hover .flip-card:active .flip-card-inner {
		transform: rotate(180deg);
	}

	.flip-card-front,
	.flip-card-back {
		-webkit-backface-visibility: hidden;
		backface-visibility: hidden;
	}

	.flip-card-back {
		transform: rotateY(180deg);
	}
</style>
