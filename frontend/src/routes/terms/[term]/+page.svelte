<script lang="ts">
	import { page } from '$app/stores';
	import Tabs from '../../../components/Tabs.svelte';
	import BriefDescription from '../../../components/termComponents/TermBrief/+page.svelte';
	import FullDescription from '../../../components/termComponents/TermFull/+page.svelte';
	import Examples from '../../../components/termComponents/TermExamples/+page.svelte';
	import Resources from '../../../components/termComponents/TermResources/+page.svelte';

	let term = $page.params.term;

	let propDetails = {
		brief:
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Nihil veniam, labore facilis laboriosam dolor dolorum illum explicabo! Omnis, eveniet eaque!\nLorem ipsum dolor sit, amet consectetur adipisicing elit. Nihil veniam, labore facilis laboriosam dolor dolorum illum explicabo! Omnis, eveniet eaque!',
		bulletPoints: [
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.',
			'Lorem ipsum dolor sit amet consectetur adipisicing.'
		],
		full: 'The 20% of economics that will yield 80% of the desired results are:\n\n1. Microeconomics: This is the study of individual economic agents such as households, firms, and markets. It helps in understanding how these agents make decisions and interact with each other.\n\n2. Macroeconomics: This is the study of the economy as a whole, including topics such as inflation, unemployment, and economic growth. It helps in understanding the overall performance of the economy.\n\n3. International trade: This is the study of trade between countries and the effects of globalization on the economy. It helps in understanding the benefits and costs of international trade.\n\n4. Monetary policy: This is the study of how central banks manage the money supply and interest rates to achieve economic goals such as price stability and full employment. It helps in understanding the role of monetary policy in the economy.\n\n5. Fiscal policy: This is the study of how governments use taxation and spending to achieve economic goals such as economic growth and stability. It helps in understanding the role of fiscal policy in the economy.\n\n6. Economic development: This is the study of how countries can achieve economic growth and development. It helps in understanding the factors that contribute to economic development.\n\n7. Game theory: This is the study of strategic decision-making in situations where the outcome depends on the actions of others. It helps in understanding how individuals and firms make decisions in competitive environments.\n\n8. Econometrics: This is the application of statistical methods to economic data. It helps in understanding how to analyze and interpret economic data.\n\n9. Behavioral economics: This is the study of how psychological factors influence economic decision-making. It helps in understanding how individuals and firms make decisions in real-world situations.\n\n10. Environmental economics: This is the study of how economic activity affects the environment and how environmental policies can be designed to achieve environmental goals. It helps in understanding the trade-offs between economic growth and environmental protection.\n\nTo master these topics, a focused learning plan could include:\n\n1. Reading textbooks and academic papers on each topic.\n\n2. Taking online courses or attending lectures on each topic.\n\n3. Practicing problem sets and case studies to apply the concepts learned.\n\n4. Participating in discussions and debates on each topic to gain a deeper understanding.\n\n5. Conducting independent research on specific topics of interest.\n\n6. Attending conferences and seminars to stay up-to-date on the latest research and trends.\n\n7. Seeking mentorship or guidance from experts in the field.\n\n8. Applying the concepts learned to real-world situations through internships or job opportunities.\n\nBy following this focused learning plan, one can gain a comprehensive understanding of the most important concepts in economics and be well-equipped to apply them in various settings.',
		examples: [
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.',
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.',
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.',
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.',
			'Lorem ipsum dolor sit amet consectetur adipisicing elit. Veritatis consequuntur, quasi recusandae reprehenderit ea error quis culpa nulla dolores porro corrupti vero non doloremque labore.'
		],
		parallels: [
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?',
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?',
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?',
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?',
			'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et nulla molestiae sed?'
		]
	};

	const webpages = [
		{ name: 'Brief', component: BriefDescription, props: { ...propDetails } },
		{ name: 'Details', component: FullDescription, props: { ...propDetails } },
		{ name: 'Examples', component: Examples, props: { ...propDetails } },
		{ name: 'Resources', component: Resources, props: { ...propDetails } }
	];
</script>

<div class="my-5">
	<h2>{term}</h2>
	<Tabs {webpages} />
</div>
