<script lang="ts">
	let bulletPointInput = '';
	let exampleInput = '';
	let parallelInput = '';

	function addPoint(e: typeof event) {
		e?.preventDefault();
		const node = document.createElement('li');
		node.innerHTML = bulletPointInput;
		document.querySelector('#bulletPointResults')?.appendChild(node);
	}

	function addExample(e: typeof event) {
		e?.preventDefault();
		const node = document.createElement('li');
		node.innerHTML = exampleInput;
		document.querySelector('#exampleResults')?.appendChild(node);
	}

	function addParallel(e: typeof event) {
		e?.preventDefault();
		const node = document.createElement('li');
		node.innerHTML = parallelInput;
		document.querySelector('#parallelResults')?.appendChild(node);
	}
</script>

<h1 class="text-center my-5 text-3xl">CRM - Term</h1>

<form action="" class="border p-5 rounded border-lime-800">
	<label for="briefDescription">Enter a brief description</label>
	<textarea type="text" name="briefDescription" class="h-36 mb-5" />

	<label for="fullDescription">Enter a full description</label>
	<textarea type="text" name="fullDescription" class="h-36 mb-5" />

	<div class="">
		<div class="">
			<div class="grid grid-cols-2">
				<div class="mb-5 w-full">
					<label for="termInput">Enter a term</label>
					<input type="text" name="termInput" class="w-full" />
				</div>
				<div class="mb-5" />
			</div>
			<label for="bulletPoint">Enter a brief bullet points</label>
			<div class="flex mb-5">
				<textarea
					name="bulletPoint"
					id="bulletPointInput"
					cols="30"
					rows="2"
					bind:value={bulletPointInput}
				/>
				<button class="w-1/12 ml-5" on:click={addPoint}>+</button>
			</div>
		</div>
		<div>
			<div class="">
				<label for="exampleInput" class="mt-5">Enter an example</label>
				<div class="flex mb-5">
					<textarea name="example" id="exampleInput" cols="30" rows="2" bind:value={exampleInput} />
					<button class="w-1/12 ml-5" on:click={addExample}>+</button>
				</div>
			</div>

			<label for="parallels" class="mt-5">Enter an parallel</label>
			<div class="flex mb-5">
				<textarea
					name="parallel"
					id="parallelInput"
					cols="30"
					rows="2"
					bind:value={parallelInput}
				/>
				<button class="w-1/12 ml-5" on:click={addParallel}>+</button>
			</div>
		</div>
	</div>
	<div class="grid grid-cols-3">
		<div>
			<label>Added bullet points</label>
			<ul id="bulletPointResults" />
		</div>

		<div>
			<label>Example results</label>
			<ul id="exampleResults" />
		</div>
		<div>
			<label>Parallel results</label>
			<ul id="parallelResults" />
		</div>
	</div>

	<button type="submit" class="mt-10 py-2 text-xl border-red-400">Submit</button>
</form>
