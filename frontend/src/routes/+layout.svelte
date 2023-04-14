<script lang="ts">
	import { Menu } from 'lucide-svelte';
	import '../app.css';
</script>

<nav class="">
	<ul>
		<li>
			<a href="/" class="secondary"><Menu /></a>
		</li>
	</ul>
	<!-- <ul>
		<li class="text-3xl"><strong>Platform</strong></li>
	</ul>
	<ul>
		<li><a href="/" class="secondary">Feedback</a></li>
	</ul> -->
</nav>

<slot />
