<script lang="ts">
	import FaPlus from 'svelte-icons/fa/FaPlus.svelte';

	let bulletPointInput = '';
	let exampleInput = '';
	let parallelInput = '';

	function addPoint(e: typeof event) {
		e?.preventDefault();
		if (bulletPointInput === '') {
			return;
		}
		const node = document.createElement('li');
		node.classList.add('chip', 'variant-filled-primary');
		node.innerHTML = bulletPointInput;
		document.querySelector('#bulletPointResults')?.appendChild(node);
		bulletPointInput = '';
	}

	function addExample(e: typeof event) {
		e?.preventDefault();
		if (exampleInput === '') {
			return;
		}
		const node = document.createElement('li');
		node.classList.add('chip', 'variant-filled-primary');
		node.innerHTML = exampleInput;
		document.querySelector('#exampleResults')?.appendChild(node);
		exampleInput = '';
	}

	function addParallel(e: typeof event) {
		e?.preventDefault();
		if (parallelInput === '') {
			return;
		}
		const node = document.createElement('li');
		node.classList.add('chip', 'variant-filled-primary');
		node.innerHTML = parallelInput;
		document.querySelector('#parallelResults')?.appendChild(node);
		parallelInput = '';
	}
</script>

<h1 class="text-center my-5 text-3xl">CRM - Term</h1>

<form action="" class="">
	<label
		><span>Enter a term </span>
		<input
			class="input"
			name="termInput"
			title="Input (text)"
			type="text"
			placeholder="input text"
		/>
	</label>
	<label class="label my-5"
		><span>Enter a brief description</span>
		<textarea
			name="briefDescription"
			class="textarea"
			rows="4"
			placeholder="Enter a brief description..."
		/>
	</label>
	<label class="label my-5"
		><span>Enter a full description</span>
		<textarea
			name="fullDescription"
			class="textarea"
			rows="4"
			placeholder="Enter a full description..."
		/>
	</label>

	<label class="my-5"
		><span>Enter a brief bullet point</span>
		<div class="flex">
			<textarea
				name="bulletPoint"
				id="bulletPointInput"
				rows="2"
				class="w-10/12 textarea"
				bind:value={bulletPointInput}
			/>
			<div class="flex flex-center p-2">
				<button type="button" class="btn-icon variant-filled" on:click={addPoint}><FaPlus /></button
				>
			</div>
		</div>
	</label>

	<label class="mt-5"
		><span>Enter an example</span>
		<div class="flex">
			<textarea
				name="example"
				id="exampleInput"
				rows="2"
				class="w-10/12 textarea"
				bind:value={exampleInput}
			/>
			<div class="flex flex-center p-2">
				<button class="btn-icon variant-filled" on:click={addExample}><FaPlus /></button>
			</div>
		</div>
	</label>

	<label for="parallels" class="mt-5">Enter an parallel</label>
	<div class="flex">
		<textarea
			name="parallel"
			id="parallelInput"
			rows="2"
			class="w-10/12 textarea"
			bind:value={parallelInput}
		/>
		<div class="flex flex-center p-2">
			<button class="btn-icon variant-filled" on:click={addParallel}><FaPlus /></button>
		</div>
	</div>

	<div class="grid grid-cols-3 my-5 gap-2">
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label
			><span class="my-5">Added bullet points</span>
			<ul id="bulletPointResults" class="list" />
		</label>

		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label
			><span class="my-5">Example results</span>
			<ul id="exampleResults" class="list" />
		</label>

		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label
			><span class="my-5">Parallel results</span>
			<ul id="parallelResults" class="list" />
		</label>
	</div>

	<button type="submit" class="btn bg-lime-600 text-white text-2xl my-5 w-full">Submit</button>
</form>
