<script lang="ts">
	import { page } from '$app/stores';
	import Tabs from '../../../components/Tabs.svelte';
	import BriefDescription from '../../../components/termComponents/TermBrief/+page.svelte';
	import FullDescription from '../../../components/termComponents/TermFull/+page.svelte';
	import Examples from '../../../components/termComponents/TermExamples/+page.svelte';
	import Resources from '../../../components/termComponents/TermResources/+page.svelte';

	let term = $page.params.term[0].toUpperCase() + $page.params.term.slice(1);

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
		],
		resources: [
			{
				id: 1,
				url: 'www.google.com',
				title: 'Google',
				category: 'Web',
				subcategory: 'Article',
				description:
					'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et',
				author: 'Some person',
				tags: ['one', 'two', 'three', 'four', 'five', 'six'],
				website: 'Google',
				length: '10'
			},
			{
				id: 2,
				url: 'www.google.com',
				title: 'Google',
				category: 'Web',
				subcategory: 'Article',
				description:
					'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Error iusto quibusdam consequuntur fugiat id minima unde et',
				author: 'Some person',
				tags: ['one', 'two', 'three', 'four', 'five', 'six'],
				website: 'Google',
				length: '10'
			}
		]
	};

	/*
		[category][sub-category][topics][authors]

			get unique list of each filter
			set as list in dropdowns for each
				need a dynamic list/dropdown component
			For all options selected, run function
				-> updates dynamic list of sources where, originalList.filter(x.[filter] in [list of selected options]


			Source page
			ul of filtered list components
			store original list
			store filtered list (same as original on load)
			list of category optons (new Set(...original.category))
			same for sub-categories
			same for tags/topics
			same for authors

			for each list of options, pass in to the dynamic component
			on selection of an option (change style later), pass the updated property to a page-level function, and update the filtered list based on the results of the options selected
			Store options selected
				- can use a hash table / object for this
				{
					topic: string,
					selected: boolean
				} 

			Then re-render the list of sources/results based on the filtered list

			Page
				- store categories chosen
				- store all categories
				- store sub-categories chosen
				- store all sub-categories
				- store topis chosedn
				- store all topics

			All options are true by default (in an array of results)
				so  the all and chosen are the same by default
				then when an option is clicked (de-selected) it gets sent to a page function that removes it from the filtered list. When clicked again, it adds it back
				later add option to select all / select none
*/

	/*let propDetails = {
		term: 'economics',
		brief:
			'Economics is the study of how people make choices about what to buy, sell, and produce. It\'s all about how we use our resources, like money, time, and materials, to get what we want and need.\n\nOne of the most important concepts in economics is supply and demand. This means that when there is a lot of something available, like a certain type of food, the price will usually be lower because there is more competition. On the other hand, when there is a limited supply of something, like a rare collectible, the price will usually be higher because people are willing to pay more for it.\n\nAnother important concept is opportunity cost. This means that when we make a choice, we have to give up something else in order to get what we want. For example, if you choose to spend your money on a new video game, you won\'t have that money to spend on something else.\n\nThere are many different types of economics, including microeconomics (which looks at individual choices and markets) and macroeconomics (which looks at the overall economy and government policies). \n\nIf you want to learn more about economics, there are many great resources available. Khan Academy has a series of videos on microeconomics and macroeconomics that are easy to understand. The book "Economics in One Lesson" by Henry Hazlitt is also a great introduction to the subject.',
		full: 'Economics is a social science that studies how individuals, businesses, governments, and societies allocate scarce resources to satisfy unlimited wants and needs. It is concerned with the production, distribution, and consumption of goods and services.\n\nThe fundamental concept of economics is scarcity. Scarcity refers to the limited availability of resources, such as land, labor, capital, and entrepreneurship, which are required to produce goods and services. Since resources are limited, individuals and societies must make choices about how to allocate them.\n\nEconomics is divided into two main branches: microeconomics and macroeconomics. Microeconomics focuses on the behavior of individuals and firms in the market, while macroeconomics studies the economy as a whole, including issues such as inflation, unemployment, and economic growth.\n\nMicroeconomics examines how individuals and firms make decisions about what to produce, how much to produce, and at what price to sell their goods and services. It also studies how markets work, including the interaction between buyers and sellers, the role of prices in allocating resources, and the impact of government policies on market outcomes.\n\nMacroeconomics, on the other hand, looks at the overall performance of the economy, including the level of output, employment, and inflation. It also studies the role of government in managing the economy, including fiscal and monetary policies.\n\nEconomists use a variety of tools and techniques to analyze economic issues. These include mathematical models, statistical analysis, and experimental methods. They also use data from a variety of sources, including government statistics, surveys, and market data.\n\nEconomics has many applications in the real world. It is used to analyze and predict market trends, to evaluate the impact of government policies, and to inform business decisions. It is also used to study social issues such as poverty, inequality, and environmental degradation.\n\nIn summary, economics is a social science that studies how individuals, businesses, governments, and societies allocate scarce resources to satisfy unlimited wants and needs. It is divided into two main branches, microeconomics and macroeconomics, and uses a variety of tools and techniques to analyze economic issues. Economics has many applications in the real world and is essential for understanding how the economy works and how to make informed decisions.',
		bulletPoints:
			'1. Microeconomics: This branch of economics deals with the behavior of individual consumers, firms, and industries. It focuses on the decisions made by these entities regarding production, consumption, and pricing. Microeconomics also examines the market structures that influence these decisions, such as perfect competition, monopolies, and oligopolies.\n\n2. Macroeconomics: This branch of economics deals with the overall performance of the economy, including factors such as inflation, unemployment, and economic growth. Macroeconomics also examines the policies that governments use to manage the economy, such as monetary and fiscal policies.\n\n3. International economics: This branch of economics deals with the interactions between different countries and their economies. It examines issues such as international trade, foreign investment, and exchange rates. International economics also looks at the policies that governments use to manage their relationships with other countries, such as trade agreements and tariffs.\n\n4. Development economics: This branch of economics deals with the economic development of countries and regions. It examines the factors that contribute to economic growth, such as education, infrastructure, and technology. Development economics also looks at the policies that governments use to promote economic development, such as foreign aid and investment incentives.\n\n5. Behavioral economics: This branch of economics deals with the psychological and social factors that influence economic decision-making. It examines how people make decisions in situations of uncertainty, and how their behavior can be influenced by factors such as social norms and cognitive biases.\n\n6. Environmental economics: This branch of economics deals with the interactions between the economy and the environment. It examines the costs and benefits of environmental policies, such as pollution taxes and emissions trading schemes. Environmental economics also looks at the ways in which economic activity can impact the natural environment, such as climate change and deforestation.\n\n7. Public economics: This branch of economics deals with the role of government in the economy. It examines the ways in which governments can use policies such as taxation and public spending to influence economic outcomes. Public economics also looks at the provision of public goods and services, such as healthcare and education.\n\n8. Financial economics: This branch of economics deals with the behavior of financial markets and institutions. It examines the pricing of financial assets such as stocks and bonds, and the ways in which financial institutions such as banks and insurance companies operate. Financial economics also looks at the role of financial regulation in ensuring the stability of the financial system.',
		examples:
			'1. Supply and demand: When there is a high demand for a product or service, the price tends to go up. Conversely, when there is a low demand, the price tends to go down.\n\n2. Opportunity cost: When you choose to do one thing, you are giving up the opportunity to do something else. For example, if you choose to go to a movie, the opportunity cost is the time and money you could have spent doing something else.\n\n3. Inflation: When the prices of goods and services increase over time, the value of money decreases. This means that the same amount of money can buy fewer goods and services than before.\n\n4. Gross Domestic Product (GDP): This is the total value of all goods and services produced in a country in a given period of time. It is used to measure the economic growth of a country.\n\n5. Taxes: These are payments made to the government by individuals and businesses to fund public services such as education, healthcare, and infrastructure.\n\n6. International trade: This refers to the exchange of goods and services between countries. It can be beneficial for both countries involved as they can specialize in producing goods they are good at and trade with other countries for goods they need.\n\n7. Market structure: This refers to the type of competition in a market. For example, a monopoly is a market structure where there is only one seller, while a perfectly competitive market has many sellers and buyers.',
		parallels:
			'1. Sports: In sports, the concept of supply and demand is evident. For example, the demand for tickets to a popular game is high, and the supply is limited, leading to higher prices. Similarly, the demand for talented players is high, and the supply is limited, leading to higher salaries.\n\n2. Cooking: In cooking, the concept of opportunity cost is evident. When choosing to cook a particular dish, the opportunity cost is the other dishes that could have been made with the same ingredients and time. Similarly, when choosing to buy a particular ingredient, the opportunity cost is the other ingredients that could have been purchased with the same money.\n\n3. Psychology: In psychology, the concept of incentives is evident. For example, people are motivated to work harder when they are offered incentives such as bonuses or promotions. Similarly, people are motivated to engage in certain behaviors when they are offered incentives such as rewards or recognition.\n\n4. Biology: In biology, the concept of scarcity is evident. For example, resources such as food and water are limited, and organisms must compete for them. Similarly, in economics, resources such as labor and capital are limited, and firms must compete for them.\n\n5. Music: In music, the concept of specialization is evident. For example, musicians specialize in playing certain instruments or genres of music. Similarly, in economics, firms specialize in producing certain goods or services.',
		sources:
			'1. Videos: \n- Khan Academy\'s Economics playlist on YouTube\n- Crash Course Economics on YouTube\n- TED Talks on economics\n- EconMovies on YouTube\n\n2. Books: \n- "Naked Economics" by Charles Wheelan\n- "Freakonomics" by Steven Levitt and Stephen Dubner\n- "The Wealth of Nations" by Adam Smith\n- "Capital in the Twenty-First Century" by Thomas Piketty\n\n3. Podcasts: \n- Planet Money by NPR\n- Freakonomics Radio by Stephen Dubner\n- The Economist Radio\n- EconTalk by Russ Roberts\n\n4. Interactive exercises: \n- EconEdLink\'s interactive lessons and activities\n- The Economics Classroom by the Council for Economic Education\n- The Economics Games by the Federal Reserve Bank of St. Louis\n- The Marginal Revolution University\'s online courses and quizzes\n\nThese resources cater to different learning styles by providing visual aids, interactive exercises, and different formats such as videos, books, and podcasts. They also cover a range of topics from basic economic principles to more complex theories and current events.',
		questions:
			'1. What is the difference between microeconomics and macroeconomics?\n2. How do supply and demand affect market prices?\n3. What is the difference between a monopoly and an oligopoly?\n4. How do externalities impact market efficiency?\n5. What is the difference between nominal and real GDP?\n6. How do changes in interest rates affect the economy?\n7. What is the difference between fiscal and monetary policy?\n8. How do international trade policies impact domestic economies?\n9. What is the difference between a progressive and regressive tax system?\n10. How do changes in income distribution affect economic growth?',
		topics:
			'1. Supply and demand\n2. Inflation\n3. Trade deficit\n4. Fiscal policy\n5. Monetary policy\n6. Market structure\n7. Labor market\n8. Economic growth\n9. International trade\n10. Consumer spending\n11. Taxation\n12. Public debt\n13. Interest rates\n14. Gross domestic product (GDP)\n15. Income inequality\n16. Cost of living\n17. Entrepreneurship\n18. Economic indicators\n19. Economic systems\n20. Price elasticity',
		wikiDescription:
			'<p class="mw-empty-elt">\n\n\n</p>\n\n<p><b>Economics</b> (<span></span>) is a social science that studies the production, distribution, and consumption of goods and services.</p><p>Economics focuses on the behaviour and interactions of economic agents and how economies work. Microeconomics analyzes what\'s viewed as basic elements in the economy, including individual agents and markets, their interactions, and the outcomes of interactions. Individual agents may include, for example, households, firms, buyers, and sellers. Macroeconomics analyzes the economy as a system where production, consumption, saving, and investment interact, and factors affecting it: employment of the resources of labour, capital, and land, currency inflation, economic growth, and public policies that have impact on these elements.\n</p><p>Other broad distinctions within economics include those between positive economics, describing "what is", and normative economics, advocating "what ought to be"; between economic theory and applied economics; between rational and behavioural economics; and between mainstream economics and heterodox economics.</p><p>Economic analysis can be applied throughout society, including business, finance, cybersecurity, health care, engineering and government. It is also applied to such diverse subjects as crime, education, the family, feminism, law, philosophy, politics, religion, social institutions, war, science, and the environment.</p>',
		wikiurl: 'https://en.wikipedia.org/wiki/Economics'
	};
*/
	const webpages = [
		{ name: 'Brief', component: BriefDescription, props: { ...propDetails } },
		{ name: 'Details', component: FullDescription, props: { ...propDetails } },
		{ name: 'Examples', component: Examples, props: { ...propDetails } },
		{ name: 'Resources', component: Resources, props: { ...propDetails } }
	];
</script>

<div class="my-5">
	<h2 class="my-5 text-4xl text-center">{term}</h2>
	<Tabs {webpages} />
</div>
