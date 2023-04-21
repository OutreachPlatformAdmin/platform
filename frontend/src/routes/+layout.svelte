<script lang="ts">
	import { Menu } from 'lucide-svelte';
	import '../app.css';
</script>

<nav class=" bg-slate-800 px-5 rounded-md lg:w-8/12 flex flex-row mx-auto">
	<ul>
		<li>
			<a href="/" class="secondary"><Menu /></a>
		</li>
	</ul>
	<ul>
		<li class="text-3xl"><strong>Platform</strong></li>
	</ul>
	<ul>
		<li>
			<a
				href="/startHere"
				role="button"
				class="bg-lime-800 secondary text-white font-bold border-none">Start Here</a
			>
		</li>
	</ul>
</nav>

<slot />
