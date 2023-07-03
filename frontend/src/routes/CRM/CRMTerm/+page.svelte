<script lang="ts">
	import FaPlus from 'svelte-icons/fa/FaPlus.svelte';

	type FormFields = {
		name: String;
		is_verified: boolean;
		brief_description: string;
		full_description: string;
		bullet_points: string[];
		examples: string[];
		parallels: string[];
	};

	let formFields: FormFields = {
		name: '',
		is_verified: false,
		brief_description: '',
		full_description: '',
		bullet_points: [''],
		examples: [''],
		parallels: ['']
	};

	function addPoint(e: typeof event) {
		e?.preventDefault();

		// Get the current value of the input for new bullet points
		let val = (document.querySelector('#bulletPointInput') as HTMLInputElement)?.value;

		// Handle adding the value to existing bullet points
		if (val === '') {
			return;
		} else if (formFields.bullet_points[0] === '') {
			formFields.bullet_points[0] = val;
		} else {
			formFields.bullet_points.push(val);
		}

		// Crreat and add the new bullet point to the UI and reset the value to empty
		const node = document.createElement('li');
		node.classList.add('chip', 'variant-filled-primary');
		node.innerHTML = val || '';

		document.querySelector('#bulletPointResults')?.appendChild(node);
		(document.querySelector('#bulletPointInput') as HTMLInputElement).value = '';
	}

	function addExample(e: typeof event) {
		e?.preventDefault();

		// Get the current value of the input for new example
		let val = (document.querySelector('#exampleInput') as HTMLInputElement)?.value;

		// Handle adding the value to existing examples
		if (val === '') {
			return;
		} else if (formFields.examples[0] === '') {
			formFields.examples[0] = val;
		} else {
			formFields.examples.push(val);
		}

		// Crreat and add the new example to the UI and reset the value to empty
		const node = document.createElement('li');
		node.classList.add('chip', 'variant-filled-primary');
		node.innerHTML = val || '';

		document.querySelector('#exampleResults')?.appendChild(node);
		(document.querySelector('#exampleInput') as HTMLInputElement).value = '';
	}

	function addParallel(e: typeof event) {
		e?.preventDefault();

		// Get the current value of the input for new parallel
		let val = (document.querySelector('#parallelInput') as HTMLInputElement)?.value;

		// Handle adding the value to existing parallels
		if (val === '') {
			return;
		} else if (formFields.parallels[0] === '') {
			formFields.parallels[0] = val;
		} else {
			formFields.parallels.push(val);
		}

		// Crreat and add the new parallel to the UI and reset the value to empty
		const node = document.createElement('li');
		node.classList.add('chip', 'variant-filled-primary');
		node.innerHTML = val || '';

		document.querySelector('#parallelResults')?.appendChild(node);
		(document.querySelector('#parallelInput') as HTMLInputElement).value = '';
	}

	const submitForm = async () => {
		const data = await fetch(`http://localhost:3000/new-term`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(formFields)
		});
		// const result = await data.json();
	};
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
			bind:value={formFields.name}
		/>
	</label>
	<label class="label my-5"
		><span>Enter a brief description</span>
		<textarea
			name="briefDescription"
			class="textarea"
			rows="4"
			placeholder="Enter a brief description..."
			bind:value={formFields.brief_description}
		/>
	</label>
	<label class="label my-5"
		><span>Enter a full description</span>
		<textarea
			name="fullDescription"
			class="textarea"
			rows="4"
			placeholder="Enter a full description..."
			bind:value={formFields.full_description}
		/>
	</label>

	<label class="my-5"
		><span>Enter a brief bullet point</span>
		<div class="flex">
			<textarea name="bulletPoint" id="bulletPointInput" rows="2" class="w-10/12 textarea" />
			<div class="flex flex-center p-2">
				<button type="button" class="btn-icon variant-filled" on:click={addPoint}><FaPlus /></button
				>
			</div>
		</div>
	</label>

	<label class="mt-5"
		><span>Enter an example</span>
		<div class="flex">
			<textarea name="example" id="exampleInput" rows="2" class="w-10/12 textarea" />
			<div class="flex flex-center p-2">
				<button class="btn-icon variant-filled" on:click={addExample}><FaPlus /></button>
			</div>
		</div>
	</label>

	<label for="parallels" class="mt-5">Enter an parallel</label>
	<div class="flex">
		<textarea name="parallel" id="parallelInput" rows="2" class="w-10/12 textarea" />
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

	<button
		on:click|preventDefault={submitForm}
		type="submit"
		class="btn bg-lime-600 text-white text-2xl my-5 w-full">Submit</button
	>
</form>
