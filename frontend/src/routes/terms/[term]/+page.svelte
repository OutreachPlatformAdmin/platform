<script lang="ts">
	import { page } from '$app/stores';
	let term = $page.params.term;
</script>

<div>
	<h1>{term}</h1>

	<!-- tab container component -->
	<div class="max-h-20 relative">
		<nav
			class="my-5 border border-lime-300 flex-grow-0 flex-shrink-0 flex-auto sticky left-0 top-0"
		>
			<ul class="w-full grid grid-cols-4">
				<li class="rounded-tr-2xl p-2 bg-slate-700 w-full hover:bg-lime-900 hover:pointer">
					<a href="">Brief</a>
				</li>
				<li class="rounded-tr-2xl p-2 bg-slate-700 w-full hover:bg-lime-900">
					<a href="">Detailed</a>
				</li>
				<li class="rounded-tr-2xl p-2 bg-slate-700 w-full hover:bg-lime-900">
					<a href="">Examples</a>
				</li>
				<li class="rounded-tr-2xl p-2 bg-slate-700 w-full hover:bg-lime-900">
					<a href="">Resources</a>
				</li>
			</ul>
		</nav>
		<p class="absolute left-0 top-20">
			Lorem ipsum dolor sit amet consectetur adipisicing elit. Maxime odit explicabo iusto
			doloremque aliquid nam, pariatur expedita debitis, ullam ut voluptatem voluptate aspernatur
			inventore! Animi eum tempore molestiae vero. Quaerat quasi totam sit suscipit fugiat ipsam
			placeat eos aut numquam. Iure dolorum quisquam esse expedita reiciendis? Natus recusandae quis
			eveniet mollitia placeat atque adipisci maxime! Alias ut, adipisci dolores, eius dignissimos
			nobis voluptas earum amet nemo aperiam impedit similique itaque illo. Architecto placeat
			voluptatem quasi aut doloribus ad nam inventore voluptatum omnis beatae earum nesciunt
			similique, modi exercitationem ea veniam blanditiis, corporis temporibus perspiciatis cum
			provident. Dolores explicabo voluptatibus earum doloribus quisquam totam omnis atque
			perferendis deleniti, quam assumenda exercitationem neque, vitae laboriosam vel repellendus
			sapiente? Lorem ipsum dolor sit amet consectetur adipisicing elit. Aliquam eius unde odit
			doloribus tempore nemo, explicabo placeat voluptatum fugit necessitatibus facilis culpa ab
			quas consequuntur ipsa quidem fugiat sed voluptatem iusto amet! Officiis odit dolor molestiae
			nobis. Voluptas perferendis perspiciatis enim reprehenderit numquam possimus fugiat sit
			recusandae maxime! Minima accusamus repellendus nobis aliquid vel ea quo. Suscipit neque
			voluptates ullam obcaecati provident repudiandae officiis iure nisi rem dolores expedita
			consectetur sunt laborum quae quod corporis, officia minus animi molestiae aliquam nostrum,
			tempora debitis temporibus? Quas, harum optio asperiores vero quos fugit, id quae esse,
			quisquam corporis voluptatem? Quibusdam exercitationem nihil nostrum fugiat distinctio quasi
			ducimus laboriosam veritatis sint, temporibus laudantium, veniam, aliquid similique corrupti
			ad amet nam iure molestiae aut suscipit qui aspernatur officia rem. Nulla minima praesentium
			quas, nihil excepturi aperiam ut totam veritatis ad, similique libero in repellat aliquam
			dignissimos id. Alias consequuntur maiores fugiat magni obcaecati quae esse doloremque atque.
			Nihil optio totam, minus, maxime rerum delectus porro ipsa voluptas corrupti blanditiis
			ducimus? Enim corrupti tempora numquam itaque, voluptas nobis ipsa ducimus deleniti ipsam,
			cupiditate ut sapiente! Sapiente, nam? Ad voluptates dolor nobis, voluptatum voluptatem amet
			optio illo repellat, molestias nulla repudiandae! Hic harum tempore delectus voluptates? Lorem
			ipsum, dolor sit amet consectetur adipisicing elit. Esse officia itaque inventore vero
			deserunt, ab nostrum consectetur vitae similique eaque neque sunt iste facere nemo impedit
			nobis nesciunt corporis enim, mollitia modi distinctio. Doloribus ut dignissimos illo
			aspernatur modi, maxime perspiciatis voluptatem, animi, excepturi dolorum quod. Perspiciatis
			reiciendis, atque doloribus velit fugiat sed quae veritatis eaque odit minima vitae nesciunt
			debitis similique ad ipsa necessitatibus. Quasi tenetur, similique impedit est excepturi animi
			quas explicabo adipisci tempora dolore, iusto voluptate, iste quibusdam. Ipsum maiores itaque
			iste? Aperiam velit dolor id alias nobis optio saepe tempora aut dignissimos atque dolorum
			tenetur, sint iure, enim laudantium dolorem dicta deleniti ut molestias. Libero esse odio
			voluptatum veritatis autem corporis itaque quos, doloribus nisi minus alias temporibus quae!
			Error cumque nihil eos expedita officia eaque nulla ipsum beatae vel eius accusantium libero,
			ea odio optio explicabo hic molestiae numquam, nisi facere in officiis sed magni quam placeat.
			Esse quas labore fuga architecto sint perferendis obcaecati commodi? Perspiciatis,
			exercitationem? Repellendus hic ipsa porro alias voluptas eius expedita labore, maxime vitae
			nobis tempora a? Facilis cupiditate aliquam non aliquid ipsa, laudantium quas adipisci dolor
			dicta ut, aut similique nisi voluptates nihil soluta ex saepe laborum id delectus.
		</p>
	</div>
</div>

<!-- <pre>
    {JSON.stringify($page, null, 2)}
</pre> -->
