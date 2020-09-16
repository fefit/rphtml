use chtml::parser::*;
fn main() {
	let mut doc = Doc::new(ParserType::HTML);
	let html = r###"
	<!DOCTYPE html>
	<html lang="en">
		<head>
			<meta charset="utf-8">
		<link rel="dns-prefetch" href="https://github.githubassets.com">
		<link rel="dns-prefetch" href="https://avatars0.githubusercontent.com">
		<link rel="dns-prefetch" href="https://avatars1.githubusercontent.com">
		<link rel="dns-prefetch" href="https://avatars2.githubusercontent.com">
		<link rel="dns-prefetch" href="https://avatars3.githubusercontent.com">
		<link rel="dns-prefetch" href="https://github-cloud.s3.amazonaws.com">
		<link rel="dns-prefetch" href="https://user-images.githubusercontent.com/">
	
	
	
		<link crossorigin="anonymous" media="all" integrity="sha512-lC+Z9kBc6E9r9umj6DgEEoQP7CX8RgGJGegRUJbthY1Bus2eemD1Kkc1Wbacj7hxeuvVCuyeqfNsKZWxqt1uIw==" rel="stylesheet" href="https://github.githubassets.com/assets/frameworks-942f99f6405ce84f6bf6e9a3e8380412.css" />
		
			<link crossorigin="anonymous" media="all" integrity="sha512-ydDNscjQE90aIkGJxc29Lb7zRSOiSDZf1QGfsxZCUb3oP6BBqZRezQY0sWYHmYA8Kk/QPk4zF6IpCkFhDlBN9A==" rel="stylesheet" href="https://github.githubassets.com/assets/github-c9d0cdb1c8d013dd1a224189c5cdbd2d.css" />
			
			
			
			
	
	
		<meta name="viewport" content="width=device-width">
		
		<title>bheisler/criterion.rs: Statistics-driven benchmarking library for Rust</title>
			<meta name="description" content="Statistics-driven benchmarking library for Rust. Contribute to bheisler/criterion.rs development by creating an account on GitHub.">
			<link rel="search" type="application/opensearchdescription+xml" href="/opensearch.xml" title="GitHub">
		<link rel="fluid-icon" href="https://github.com/fluidicon.png" title="GitHub">
		<meta property="fb:app_id" content="1401488693436528">
		<meta name="apple-itunes-app" content="app-id=1477376905">
	
			<meta name="twitter:image:src" content="https://avatars1.githubusercontent.com/u/1616938?s=400&amp;v=4" /><meta name="twitter:site" content="@github" /><meta name="twitter:card" content="summary" /><meta name="twitter:title" content="bheisler/criterion.rs" /><meta name="twitter:description" content="Statistics-driven benchmarking library for Rust. Contribute to bheisler/criterion.rs development by creating an account on GitHub." />
			<meta property="og:image" content="https://avatars1.githubusercontent.com/u/1616938?s=400&amp;v=4" /><meta property="og:site_name" content="GitHub" /><meta property="og:type" content="object" /><meta property="og:title" content="bheisler/criterion.rs" /><meta property="og:url" content="https://github.com/bheisler/criterion.rs" /><meta property="og:description" content="Statistics-driven benchmarking library for Rust. Contribute to bheisler/criterion.rs development by creating an account on GitHub." />
	
	
	
		
	
		<link rel="assets" href="https://github.githubassets.com/">
			<link rel="shared-web-socket" href="wss://alive.github.com/_sockets/u/7714080/ws?session=eyJ2IjoiVjMiLCJ1Ijo3NzE0MDgwLCJzIjo1NTE4Mjg4NDUsImMiOjQxMzk3ODE4NzksInQiOjE2MDAyNTE3MjV9--66be6e965a29d333d5a15e5582750fbe4c55962d75e3f5740dbfe303b60ee5f5" data-refresh-url="/_alive">
		<link rel="sudo-modal" href="/sessions/sudo_modal">
	
		<meta name="request-id" content="E2C7:128D:139EFD5:1D90745:5F61E735" data-pjax-transient="true" /><meta name="html-safe-nonce" content="bdd9766b44bc98947e41af87bd275723e3ff0b3b" data-pjax-transient="true" /><meta name="visitor-payload" content="eyJyZWZlcnJlciI6Imh0dHBzOi8vd3d3Lmdvb2dsZS5jb20uaGsvIiwicmVxdWVzdF9pZCI6IkUyQzc6MTI4RDoxMzlFRkQ1OjFEOTA3NDU6NUY2MUU3MzUiLCJ2aXNpdG9yX2lkIjoiNTM0ODY3MjQ3NjMzNDMzNDIyMyIsInJlZ2lvbl9lZGdlIjoiaWFkIiwicmVnaW9uX3JlbmRlciI6ImlhZCJ9" data-pjax-transient="true" /><meta name="visitor-hmac" content="72ea2be3ab401a98dfac749968ba01db126f1f792b660f2b8341d04835c25bb5" data-pjax-transient="true" /><meta name="cookie-consent-required" content="false" />
	
			<meta name="hovercard-subject-tag" content="repository:20188259" data-pjax-transient>
	
	
		<meta name="github-keyboard-shortcuts" content="repository" data-pjax-transient="true" />
	
		
	
		<meta name="selected-link" value="repo_source" data-pjax-transient>
	
			<meta name="google-site-verification" content="c1kuD-K2HIVF635lypcsWPoD4kilo5-jA_wBFyT4uMY">
		<meta name="google-site-verification" content="KT5gs8h0wvaagLKAVWq8bbeNwnZZK1r1XQysX3xurLU">
		<meta name="google-site-verification" content="ZzhVyEFwb7w3e0-uOTltm8Jsck2F5StVihD0exw2fsA">
		<meta name="google-site-verification" content="GXs5KoUUkNCoaAZn7wPN-t01Pywp9M3sEjnt_3_ZWPc">
	
		<meta name="octolytics-host" content="collector.githubapp.com" /><meta name="octolytics-app-id" content="github" /><meta name="octolytics-event-url" content="https://collector.githubapp.com/github-external/browser_event" /><meta name="octolytics-dimension-ga_id" content="" class="js-octo-ga-id" /><meta name="octolytics-actor-id" content="7714080" /><meta name="octolytics-actor-login" content="fefit" /><meta name="octolytics-actor-hash" content="1b58ca152944f73f1948dda127bf9175888088c29c2dd11c592f0fd5b4426738" />
	
		<meta name="analytics-location" content="/&lt;user-name&gt;/&lt;repo-name&gt;" data-pjax-transient="true" />
	
		
	
	
	
	
	
			<meta name="google-analytics" content="UA-3769691-2">
	
		<meta class="js-ga-set" name="userId" content="6b252d6feba4cb7c58494065762a9d47">
	
	<meta class="js-ga-set" name="dimension10" content="Responsive" data-pjax-transient>
	
	<meta class="js-ga-set" name="dimension1" content="Logged In">
	
	
	
		
	
				<meta name="hostname" content="github.com">
			<meta name="user-login" content="fefit">
	
	
				<meta name="expected-hostname" content="github.com">
	
				<meta name="js-proxy-site-detection-payload" content="MzBiOTBhYjEwZjY4M2FmYTdjOTIwZjA2YTQ0MTZkODllMjRmNTAyNjkxZDNhN2EzMjI5MDY1ZmJhZGQ3NmQ2N3x7InJlbW90ZV9hZGRyZXNzIjoiMTA0LjE5Mi4xMDguMTAiLCJyZXF1ZXN0X2lkIjoiRTJDNzoxMjhEOjEzOUVGRDU6MUQ5MDc0NTo1RjYxRTczNSIsInRpbWVzdGFtcCI6MTYwMDI1MTcyNSwiaG9zdCI6ImdpdGh1Yi5jb20ifQ==">
	
			<meta name="enabled-features" content="MARKETPLACE_PENDING_INSTALLATIONS,JS_HTTP_CACHE_HEADERS">
	
		<meta http-equiv="x-pjax-version" content="7672c080366cd68c0eba0deb84ecb550">
		
	
					<link href="https://github.com/bheisler/criterion.rs/commits/master.atom" rel="alternate" title="Recent Commits to criterion.rs:master" type="application/atom+xml">
	
		<meta name="go-import" content="github.com/bheisler/criterion.rs git https://github.com/bheisler/criterion.rs.git">
	
		<meta name="octolytics-dimension-user_id" content="1616938" /><meta name="octolytics-dimension-user_login" content="bheisler" /><meta name="octolytics-dimension-repository_id" content="20188259" /><meta name="octolytics-dimension-repository_nwo" content="bheisler/criterion.rs" /><meta name="octolytics-dimension-repository_public" content="true" /><meta name="octolytics-dimension-repository_is_fork" content="false" /><meta name="octolytics-dimension-repository_network_root_id" content="20188259" /><meta name="octolytics-dimension-repository_network_root_nwo" content="bheisler/criterion.rs" /><meta name="octolytics-dimension-repository_explore_github_marketplace_ci_cta_shown" content="false" />
	
	
			<link rel="canonical" href="https://github.com/bheisler/criterion.rs" data-pjax-transient>
	
	
		<meta name="browser-stats-url" content="https://api.github.com/_private/browser/stats">
	
		<meta name="browser-errors-url" content="https://api.github.com/_private/browser/errors">
	
		<link rel="mask-icon" href="https://github.githubassets.com/pinned-octocat.svg" color="#000000">
		<link rel="alternate icon" class="js-site-favicon" type="image/png" href="https://github.githubassets.com/favicons/favicon.png">
		<link rel="icon" class="js-site-favicon" type="image/svg+xml" href="https://github.githubassets.com/favicons/favicon.svg">
	
	<meta name="theme-color" content="#1e2327">
	
	
		<link rel="manifest" href="/manifest.json" crossOrigin="use-credentials">
	
		</head>
	
		<body class="logged-in env-production page-responsive">
			
	
			<div class="position-relative js-header-wrapper ">
				<a href="#start-of-content" class="p-3 bg-blue text-white show-on-focus js-skip-to-content">Skip to content</a>
				<span class="progress-pjax-loader width-full js-pjax-loader-bar Progress position-fixed">
			<span style="background-color: #79b8ff;width: 0%;" class="Progress-item progress-pjax-loader-bar "></span>
	</span>      
				
	
	
	
						<header class="Header py-md-0 js-details-container Details flex-wrap flex-md-nowrap px-3" role="banner">
		<div class="Header-item d-none d-md-flex">
			<a class="Header-link" href="https://github.com/" data-hotkey="g d"
		aria-label="Homepage " data-ga-click="Header, go to dashboard, icon:logo">
		<svg class="octicon octicon-mark-github v-align-middle" height="32" viewBox="0 0 16 16" version="1.1" width="32" aria-hidden="true"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
	</a>
	
		</div>
	
		<div class="Header-item d-md-none">
			<button class="Header-link btn-link js-details-target" type="button" aria-label="Toggle navigation" aria-expanded="false">
				<svg height="24" class="octicon octicon-three-bars" viewBox="0 0 16 16" version="1.1" width="24" aria-hidden="true"><path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 2.75zm0 5A.75.75 0 011.75 7h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 7.75zM1.75 12a.75.75 0 100 1.5h12.5a.75.75 0 100-1.5H1.75z"></path></svg>
			</button>
		</div>
	
		<div class="Header-item Header-item--full flex-column flex-md-row width-full flex-order-2 flex-md-order-none mr-0 mr-md-3 mt-3 mt-md-0 Details-content--hidden-not-important d-md-flex">
					<div hidden class="d-none">
	</div>
	<div class="header-search header-search-current js-header-search-current flex-auto  flex-self-stretch flex-md-self-auto mr-0 mr-md-3 mb-3 mb-md-0 scoped-search site-scoped-search js-site-search position-relative js-jump-to js-header-search-current-jump-to "
		role="combobox"
		aria-owns="jump-to-results"
		aria-label="Search or jump to"
		aria-haspopup="listbox"
		aria-expanded="false"
	>
		<div class="position-relative">
			<!-- '"` --><!-- </textarea></xmp> --></option></form><form class="js-site-search-form" role="search" aria-label="Site" data-scope-type="Repository" data-scope-id="20188259" data-scoped-search-url="/bheisler/criterion.rs/search" data-unscoped-search-url="/search" action="/bheisler/criterion.rs/search" accept-charset="UTF-8" method="get">
				<label class="form-control input-sm header-search-wrapper p-0 header-search-wrapper-jump-to position-relative d-flex flex-justify-between flex-items-center js-chromeless-input-container">
					<input type="text"
						class="form-control input-sm header-search-input jump-to-field js-jump-to-field js-site-search-focus js-site-search-field is-clearable"
						data-hotkey="s,/"
						name="q"
						value=""
						placeholder="Search or jump to…"
						data-unscoped-placeholder="Search or jump to…"
						data-scoped-placeholder="Search or jump to…"
						autocapitalize="off"
						aria-autocomplete="list"
						aria-controls="jump-to-results"
						aria-label="Search or jump to…"
						data-jump-to-suggestions-path="/_graphql/GetSuggestedNavigationDestinations"
						spellcheck="false"
						autocomplete="off"
						>
						<input type="hidden" value="oUcj2hK4ZRI7cdYAqpT/FSOU13JxLqoD5qjkUsu56g3owABUDyMnHOj8U5g/yN3/hF26WAU9jSBgjDvCmULrTA==" data-csrf="true" class="js-data-jump-to-suggestions-path-csrf" />
						<input type="hidden" class="js-site-search-type-field" name="type" >
							<img src="https://github.githubassets.com/images/search-key-slash.svg" alt="" class="mr-2 header-search-key-slash">
	
							<div class="Box position-absolute overflow-hidden d-none jump-to-suggestions js-jump-to-suggestions-container">
								
	<ul class="d-none js-jump-to-suggestions-template-container">
		
	
	<li class="d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-suggestion" role="option">
		<a tabindex="-1" class="no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2" href="">
			<div class="jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none">
				<svg height="16" width="16" class="octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none" title="Repository" aria-label="Repository" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
				<svg height="16" width="16" class="octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none" title="Project" aria-label="Project" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z"></path></svg>
				<svg height="16" width="16" class="octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none" title="Search" aria-label="Search" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z"></path></svg>
			</div>
	
			<img class="avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none" alt="" aria-label="Team" src="" width="28" height="28">
	
			<div class="jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target">
			</div>
	
			<div class="border rounded-1 flex-shrink-0 bg-gray px-1 text-gray-light ml-1 f6 d-none js-jump-to-badge-search">
				<span class="js-jump-to-badge-search-text-default d-none" aria-label="in this repository">
					In this repository
				</span>
				<span class="js-jump-to-badge-search-text-global d-none" aria-label="in all of GitHub">
					All GitHub
				</span>
				<span aria-hidden="true" class="d-inline-block ml-1 v-align-middle">↵</span>
			</div>
	
			<div aria-hidden="true" class="border rounded-1 flex-shrink-0 bg-gray px-1 text-gray-light ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump">
				Jump to
				<span class="d-inline-block ml-1 v-align-middle">↵</span>
			</div>
		</a>
	</li>
	
	</ul>
	
	<ul class="d-none js-jump-to-no-results-template-container">
		<li class="d-flex flex-justify-center flex-items-center f5 d-none js-jump-to-suggestion p-2">
			<span class="text-gray">No suggested jump to results</span>
		</li>
	</ul>
	
	<ul id="jump-to-results" role="listbox" class="p-0 m-0 js-navigation-container jump-to-suggestions-results-container js-jump-to-suggestions-results-container">
		
	
	<li class="d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-scoped-search d-none" role="option">
		<a tabindex="-1" class="no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2" href="">
			<div class="jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none">
				<svg height="16" width="16" class="octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none" title="Repository" aria-label="Repository" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
				<svg height="16" width="16" class="octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none" title="Project" aria-label="Project" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z"></path></svg>
				<svg height="16" width="16" class="octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none" title="Search" aria-label="Search" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z"></path></svg>
			</div>
	
			<img class="avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none" alt="" aria-label="Team" src="" width="28" height="28">
	
			<div class="jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target">
			</div>
	
			<div class="border rounded-1 flex-shrink-0 bg-gray px-1 text-gray-light ml-1 f6 d-none js-jump-to-badge-search">
				<span class="js-jump-to-badge-search-text-default d-none" aria-label="in this repository">
					In this repository
				</span>
				<span class="js-jump-to-badge-search-text-global d-none" aria-label="in all of GitHub">
					All GitHub
				</span>
				<span aria-hidden="true" class="d-inline-block ml-1 v-align-middle">↵</span>
			</div>
	
			<div aria-hidden="true" class="border rounded-1 flex-shrink-0 bg-gray px-1 text-gray-light ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump">
				Jump to
				<span class="d-inline-block ml-1 v-align-middle">↵</span>
			</div>
		</a>
	</li>
	
		
	
	<li class="d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-global-search d-none" role="option">
		<a tabindex="-1" class="no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2" href="">
			<div class="jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none">
				<svg height="16" width="16" class="octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none" title="Repository" aria-label="Repository" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
				<svg height="16" width="16" class="octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none" title="Project" aria-label="Project" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z"></path></svg>
				<svg height="16" width="16" class="octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none" title="Search" aria-label="Search" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z"></path></svg>
			</div>
	
			<img class="avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none" alt="" aria-label="Team" src="" width="28" height="28">
	
			<div class="jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target">
			</div>
	
			<div class="border rounded-1 flex-shrink-0 bg-gray px-1 text-gray-light ml-1 f6 d-none js-jump-to-badge-search">
				<span class="js-jump-to-badge-search-text-default d-none" aria-label="in this repository">
					In this repository
				</span>
				<span class="js-jump-to-badge-search-text-global d-none" aria-label="in all of GitHub">
					All GitHub
				</span>
				<span aria-hidden="true" class="d-inline-block ml-1 v-align-middle">↵</span>
			</div>
	
			<div aria-hidden="true" class="border rounded-1 flex-shrink-0 bg-gray px-1 text-gray-light ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump">
				Jump to
				<span class="d-inline-block ml-1 v-align-middle">↵</span>
			</div>
		</a>
	</li>
	
	
			<li class="d-flex flex-justify-center flex-items-center p-0 f5 js-jump-to-suggestion">
				<img src="https://github.githubassets.com/images/spinners/octocat-spinner-128.gif" alt="Octocat Spinner Icon" class="m-2" width="28">
			</li>
	</ul>
	
							</div>
				</label>
	</form>  </div>
	</div>
	
	
			<nav class="d-flex flex-column flex-md-row flex-self-stretch flex-md-self-auto" aria-label="Global">
			<a class="Header-link py-md-3 d-block d-md-none py-2 border-top border-md-top-0 border-white-fade-15" data-ga-click="Header, click, Nav menu - item:dashboard:user" aria-label="Dashboard" href="/dashboard">
				Dashboard
	</a>
		<a class="js-selected-navigation-item Header-link py-md-3  mr-0 mr-md-3 py-2 border-top border-md-top-0 border-white-fade-15" data-hotkey="g p" data-ga-click="Header, click, Nav menu - item:pulls context:user" aria-label="Pull requests you created" data-selected-links="/pulls /pulls/assigned /pulls/mentioned /pulls" href="/pulls">
				Pull<span class="d-inline d-md-none d-lg-inline"> request</span>s
	</a>
		<a class="js-selected-navigation-item Header-link py-md-3  mr-0 mr-md-3 py-2 border-top border-md-top-0 border-white-fade-15" data-hotkey="g i" data-ga-click="Header, click, Nav menu - item:issues context:user" aria-label="Issues you created" data-selected-links="/issues /issues/assigned /issues/mentioned /issues" href="/issues">
			Issues
	</a>
	
			<div class="mr-0 mr-md-3 py-2 py-md-0 border-top border-md-top-0 border-white-fade-15">
				<a class="js-selected-navigation-item Header-link py-md-3 d-inline-block" data-ga-click="Header, click, Nav menu - item:marketplace context:user" data-octo-click="marketplace_click" data-octo-dimensions="location:nav_bar" data-selected-links=" /marketplace" href="/marketplace">
					Marketplace
	</a>      
	
			</div>
	
		<a class="js-selected-navigation-item Header-link py-md-3  mr-0 mr-md-3 py-2 border-top border-md-top-0 border-white-fade-15" data-ga-click="Header, click, Nav menu - item:explore" data-selected-links="/explore /trending /trending/developers /integrations /integrations/feature/code /integrations/feature/collaborate /integrations/feature/ship showcases showcases_search showcases_landing /explore" href="/explore">
			Explore
	</a>
	
	
			<a class="Header-link d-block d-md-none mr-0 mr-md-3 py-2 py-md-3 border-top border-md-top-0 border-white-fade-15" href="/fefit">
				<img class="avatar avatar-user" src="https://avatars2.githubusercontent.com/u/7714080?s=40&amp;v=4" width="20" height="20" alt="@fefit" />
				fefit
	</a>
			<!-- '"` --><!-- </textarea></xmp> --></option></form><form action="/logout" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="imqSgVkAMDwzZEUIE97JpyZiSl3nvpU1j9o0HlQzjSYFRPOpAUxEB8SYpgj5wujICMpoatxUgczdaj6NTZLxJA==" />
				<button type="submit" class="Header-link mr-0 mr-md-3 py-2 py-md-3 border-top border-md-top-0 border-white-fade-15 d-md-none btn-link d-block width-full text-left" data-ga-click="Header, sign out, icon:logout" style="padding-left: 2px;">
					<svg class="octicon octicon-sign-out v-align-middle" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M2 2.75C2 1.784 2.784 1 3.75 1h2.5a.75.75 0 010 1.5h-2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 012 13.25V2.75zm10.44 4.5H6.75a.75.75 0 000 1.5h5.69l-1.97 1.97a.75.75 0 101.06 1.06l3.25-3.25a.75.75 0 000-1.06l-3.25-3.25a.75.75 0 10-1.06 1.06l1.97 1.97z"></path></svg>
					Sign out
				</button>
	</form></nav>
	
		</div>
	
		<div class="Header-item Header-item--full flex-justify-center d-md-none position-relative">
			<a class="Header-link" href="https://github.com/" data-hotkey="g d"
		aria-label="Homepage " data-ga-click="Header, go to dashboard, icon:logo">
		<svg class="octicon octicon-mark-github v-align-middle" height="32" viewBox="0 0 16 16" version="1.1" width="32" aria-hidden="true"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
	</a>
	
		</div>
	
		<div class="Header-item mr-0 mr-md-3 flex-order-1 flex-md-order-none">
			
	
			<notification-indicator class="js-socket-channel" data-channel="eyJjIjoibm90aWZpY2F0aW9uLWNoYW5nZWQ6NzcxNDA4MCIsInQiOjE2MDAyNTE3MjV9--fc24e6b9a5135240bed547f7ffd9ebe15c2dd38d1a3198381644a257f7777610">
				<a href="/notifications"
					 class="Header-link notification-indicator position-relative tooltipped tooltipped-sw"
					 aria-label="You have unread notifications"
					 data-hotkey="g n"
					 data-ga-click="Header, go to notifications, icon:unread"
					 data-target="notification-indicator.link">
					 <span class="mail-status unread" data-target="notification-indicator.modifier"></span>
					 <svg class="octicon octicon-bell" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z"></path><path fill-rule="evenodd" d="M8 1.5A3.5 3.5 0 004.5 5v2.947c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01l.001.006c0 .002.002.004.004.006a.017.017 0 00.006.004l.007.001h10.964l.007-.001a.016.016 0 00.006-.004.016.016 0 00.004-.006l.001-.007a.017.017 0 00-.003-.01l-1.703-2.554a1.75 1.75 0 01-.294-.97V5A3.5 3.5 0 008 1.5zM3 5a5 5 0 0110 0v2.947c0 .05.015.098.042.139l1.703 2.555A1.518 1.518 0 0113.482 13H2.518a1.518 1.518 0 01-1.263-2.36l1.703-2.554A.25.25 0 003 7.947V5z"></path></svg>
				</a>
			</notification-indicator>
	
		</div>
	
	
		<div class="Header-item position-relative d-none d-md-flex">
			<details class="details-overlay details-reset">
		<summary class="Header-link"
				aria-label="Create new…"
				data-ga-click="Header, create new, icon:add">
			<svg class="octicon octicon-plus" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 2a.75.75 0 01.75.75v4.5h4.5a.75.75 0 010 1.5h-4.5v4.5a.75.75 0 01-1.5 0v-4.5h-4.5a.75.75 0 010-1.5h4.5v-4.5A.75.75 0 018 2z"></path></svg> <span class="dropdown-caret"></span>
		</summary>
		<details-menu class="dropdown-menu dropdown-menu-sw mt-n2">
			
	<a role="menuitem" class="dropdown-item" href="/new" data-ga-click="Header, create new repository">
		New repository
	</a>
	
		<a role="menuitem" class="dropdown-item" href="/new/import" data-ga-click="Header, import a repository">
			Import repository
		</a>
	
	<a role="menuitem" class="dropdown-item" href="https://gist.github.com/" data-ga-click="Header, create new gist">
		New gist
	</a>
	
		<a role="menuitem" class="dropdown-item" href="/organizations/new" data-ga-click="Header, create new organization">
			New organization
		</a>
	
	
		<div role="none" class="dropdown-divider"></div>
		<div class="dropdown-header">
			<span title="bheisler/criterion.rs">This repository</span>
		</div>
			<a role="menuitem" class="dropdown-item" href="/bheisler/criterion.rs/issues/new/choose" data-ga-click="Header, create new issue" data-skip-pjax>
				New issue
			</a>
	
	
		</details-menu>
	</details>
	
		</div>
	
		<div class="Header-item position-relative mr-0 d-none d-md-flex">
			
		<details class="details-overlay details-reset js-feature-preview-indicator-container" data-feature-preview-indicator-src="/users/fefit/feature_preview/indicator_check">
	
		<summary class="Header-link"
			aria-label="View profile and more"
			data-ga-click="Header, show menu, icon:avatar">
			<img
		alt="@fefit"
		width="20"
		height="20"
		src="https://avatars1.githubusercontent.com/u/7714080?s=60&amp;v=4"
		class="avatar avatar-user " />
	
				<span class="feature-preview-indicator js-feature-preview-indicator" style="top: 10px;" hidden></span>
			<span class="dropdown-caret"></span>
		</summary>
		<details-menu class="dropdown-menu dropdown-menu-sw mt-n2" style="width: 180px" >
			<div class="header-nav-current-user css-truncate"><a role="menuitem" class="no-underline user-profile-link px-3 pt-2 pb-2 mb-n2 mt-n1 d-block" href="/fefit" data-ga-click="Header, go to profile, text:Signed in as">Signed in as <strong class="css-truncate-target">fefit</strong></a></div>
			<div role="none" class="dropdown-divider"></div>
	
				<div class="pl-3 pr-3 f6 user-status-container js-user-status-context lh-condensed" data-url="/users/status?compact=1&amp;link_mentions=0&amp;truncate=1">
					
	<div class="js-user-status-container rounded-1 px-2 py-1 mt-2 border"
		data-team-hovercards-enabled>
		<details class="js-user-status-details details-reset details-overlay details-overlay-dark">
			<summary class="btn-link btn-block link-gray no-underline js-toggle-user-status-edit toggle-user-status-edit "
				role="menuitem" data-hydro-click="{&quot;event_type&quot;:&quot;user_profile.click&quot;,&quot;payload&quot;:{&quot;profile_user_id&quot;:1616938,&quot;target&quot;:&quot;EDIT_USER_STATUS&quot;,&quot;user_id&quot;:7714080,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;}}" data-hydro-click-hmac="865d2df67c9d6f167af6a775df8bc4cb87781a1f36351eed0e83e0a2dcf9c209">
				<div class="d-flex flex-items-center flex-items-stretch">
					<div class="f6 lh-condensed user-status-header d-flex user-status-emoji-only-header circle">
						<div class="user-status-emoji-container flex-shrink-0 mr-2 d-flex flex-items-center flex-justify-center lh-condensed-ultra v-align-bottom">
							<svg class="octicon octicon-smiley" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zM5 8a1 1 0 100-2 1 1 0 000 2zm7-1a1 1 0 11-2 0 1 1 0 012 0zM5.32 9.636a.75.75 0 011.038.175l.007.009c.103.118.22.222.35.31.264.178.683.37 1.285.37.602 0 1.02-.192 1.285-.371.13-.088.247-.192.35-.31l.007-.008a.75.75 0 111.222.87l-.614-.431c.614.43.614.431.613.431v.001l-.001.002-.002.003-.005.007-.014.019a1.984 1.984 0 01-.184.213c-.16.166-.338.316-.53.445-.63.418-1.37.638-2.127.629-.946 0-1.652-.308-2.126-.63a3.32 3.32 0 01-.715-.657l-.014-.02-.005-.006-.002-.003v-.002h-.001l.613-.432-.614.43a.75.75 0 01.183-1.044h.001z"></path></svg>
						</div>
					</div>
					<div class="
						
						 user-status-message-wrapper f6 min-width-0"
						 style="line-height: 20px;" >
						<div class="css-truncate css-truncate-target width-fit text-gray-dark text-left">
								<span class="text-gray">Set status</span>
						</div>
					</div>
				</div>
			</summary>
			<details-dialog class="details-dialog rounded-1 anim-fade-in fast Box Box--overlay" role="dialog" tabindex="-1">
				<!-- '"` --><!-- </textarea></xmp> --></option></form><form class="position-relative flex-auto js-user-status-form" action="/users/status?circle=0&amp;compact=1&amp;link_mentions=0&amp;truncate=1" accept-charset="UTF-8" method="post"><input type="hidden" name="_method" value="put" /><input type="hidden" name="authenticity_token" value="E79Tl67zKq+CnsPfk95PwGDqtyvtCUm2rWqSRWhkHnucTzXnhdtlDSUI2+4zC2hugklYVW5I5dLVAdrg7Vidog==" />
					<div class="Box-header bg-gray border-bottom p-3">
						<button class="Box-btn-octicon js-toggle-user-status-edit btn-octicon float-right" type="reset" aria-label="Close dialog" data-close-dialog>
							<svg class="octicon octicon-x" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path></svg>
						</button>
						<h3 class="Box-title f5 text-bold text-gray-dark">Edit status</h3>
					</div>
					<input type="hidden" name="emoji" class="js-user-status-emoji-field" value="">
					<input type="hidden" name="organization_id" class="js-user-status-org-id-field" value="">
					<div class="px-3 py-2 text-gray-dark">
						<div class="js-characters-remaining-container position-relative mt-2">
							<div class="input-group d-table form-group my-0 js-user-status-form-group">
								<span class="input-group-button d-table-cell v-align-middle" style="width: 1%">
									<button type="button" aria-label="Choose an emoji" class="btn-outline btn js-toggle-user-status-emoji-picker btn-open-emoji-picker p-0">
										<span class="js-user-status-original-emoji" hidden></span>
										<span class="js-user-status-custom-emoji"></span>
										<span class="js-user-status-no-emoji-icon" >
											<svg class="octicon octicon-smiley" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zM5 8a1 1 0 100-2 1 1 0 000 2zm7-1a1 1 0 11-2 0 1 1 0 012 0zM5.32 9.636a.75.75 0 011.038.175l.007.009c.103.118.22.222.35.31.264.178.683.37 1.285.37.602 0 1.02-.192 1.285-.371.13-.088.247-.192.35-.31l.007-.008a.75.75 0 111.222.87l-.614-.431c.614.43.614.431.613.431v.001l-.001.002-.002.003-.005.007-.014.019a1.984 1.984 0 01-.184.213c-.16.166-.338.316-.53.445-.63.418-1.37.638-2.127.629-.946 0-1.652-.308-2.126-.63a3.32 3.32 0 01-.715-.657l-.014-.02-.005-.006-.002-.003v-.002h-.001l.613-.432-.614.43a.75.75 0 01.183-1.044h.001z"></path></svg>
										</span>
									</button>
								</span>
								<text-expander keys=": @" data-mention-url="/autocomplete/user-suggestions" data-emoji-url="/autocomplete/emoji">
									<input
										type="text"
										autocomplete="off"
										data-no-org-url="/autocomplete/user-suggestions"
										data-org-url="/suggestions?mention_suggester=1"
										data-maxlength="80"
										class="d-table-cell width-full form-control js-user-status-message-field js-characters-remaining-field"
										placeholder="What's happening?"
										name="message"
										value=""
										aria-label="What is your current status?">
								</text-expander>
								<div class="error">Could not update your status, please try again.</div>
							</div>
							<div style="margin-left: 53px" class="my-1 text-small label-characters-remaining js-characters-remaining" data-suffix="remaining" hidden>
								80 remaining
							</div>
						</div>
						<include-fragment class="js-user-status-emoji-picker" data-url="/users/status/emoji"></include-fragment>
						<div class="overflow-auto ml-n3 mr-n3 px-3 border-bottom" style="max-height: 33vh">
							<div class="user-status-suggestions js-user-status-suggestions collapsed overflow-hidden">
								<h4 class="f6 text-normal my-3">Suggestions:</h4>
								<div class="mx-3 mt-2 clearfix">
										<div class="float-left col-6">
												<button type="button" value=":palm_tree:" class="d-flex flex-items-baseline flex-items-stretch lh-condensed f6 btn-link link-gray no-underline js-predefined-user-status mb-1">
													<div class="emoji-status-width mr-2 v-align-middle js-predefined-user-status-emoji">
														<g-emoji alias="palm_tree" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f334.png">🌴</g-emoji>
													</div>
													<div class="d-flex flex-items-center no-underline js-predefined-user-status-message ws-normal text-left" style="border-left: 1px solid transparent">
														On vacation
													</div>
												</button>
												<button type="button" value=":face_with_thermometer:" class="d-flex flex-items-baseline flex-items-stretch lh-condensed f6 btn-link link-gray no-underline js-predefined-user-status mb-1">
													<div class="emoji-status-width mr-2 v-align-middle js-predefined-user-status-emoji">
														<g-emoji alias="face_with_thermometer" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f912.png">🤒</g-emoji>
													</div>
													<div class="d-flex flex-items-center no-underline js-predefined-user-status-message ws-normal text-left" style="border-left: 1px solid transparent">
														Out sick
													</div>
												</button>
										</div>
										<div class="float-left col-6">
												<button type="button" value=":house:" class="d-flex flex-items-baseline flex-items-stretch lh-condensed f6 btn-link link-gray no-underline js-predefined-user-status mb-1">
													<div class="emoji-status-width mr-2 v-align-middle js-predefined-user-status-emoji">
														<g-emoji alias="house" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f3e0.png">🏠</g-emoji>
													</div>
													<div class="d-flex flex-items-center no-underline js-predefined-user-status-message ws-normal text-left" style="border-left: 1px solid transparent">
														Working from home
													</div>
												</button>
												<button type="button" value=":dart:" class="d-flex flex-items-baseline flex-items-stretch lh-condensed f6 btn-link link-gray no-underline js-predefined-user-status mb-1">
													<div class="emoji-status-width mr-2 v-align-middle js-predefined-user-status-emoji">
														<g-emoji alias="dart" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f3af.png">🎯</g-emoji>
													</div>
													<div class="d-flex flex-items-center no-underline js-predefined-user-status-message ws-normal text-left" style="border-left: 1px solid transparent">
														Focusing
													</div>
												</button>
										</div>
								</div>
							</div>
							<div class="user-status-limited-availability-container">
								<div class="form-checkbox my-0">
									<input type="checkbox" name="limited_availability" value="1" class="js-user-status-limited-availability-checkbox" data-default-message="I may be slow to respond." aria-describedby="limited-availability-help-text-truncate-true-compact-true" id="limited-availability-truncate-true-compact-true">
									<label class="d-block f5 text-gray-dark mb-1" for="limited-availability-truncate-true-compact-true">
										Busy
									</label>
									<p class="note" id="limited-availability-help-text-truncate-true-compact-true">
										When others mention you, assign you, or request your review,
										GitHub will let them know that you have limited availability.
									</p>
								</div>
							</div>
						</div>
						<div class="d-inline-block f5 mr-2 pt-3 pb-2" >
		<div class="d-inline-block mr-1">
			Clear status
		</div>
	
		<details class="js-user-status-expire-drop-down f6 dropdown details-reset details-overlay d-inline-block mr-2">
			<summary class="btn btn-sm v-align-baseline" aria-haspopup="true">
				<div class="js-user-status-expiration-interval-selected d-inline-block v-align-baseline">
					Never
				</div>
				<div class="dropdown-caret"></div>
			</summary>
	
			<ul class="dropdown-menu dropdown-menu-se pl-0 overflow-auto" style="width: 220px; max-height: 15.5em">
				<li>
					<button type="button" class="btn-link dropdown-item js-user-status-expire-button ws-normal" title="Never">
						<span class="d-inline-block text-bold mb-1">Never</span>
						<div class="f6 lh-condensed">Keep this status until you clear your status or edit your status.</div>
					</button>
				</li>
				<li class="dropdown-divider" role="none"></li>
					<li>
						<button type="button" class="btn-link dropdown-item ws-normal js-user-status-expire-button" title="in 30 minutes" value="2020-09-16T18:52:05+08:00">
							in 30 minutes
						</button>
					</li>
					<li>
						<button type="button" class="btn-link dropdown-item ws-normal js-user-status-expire-button" title="in 1 hour" value="2020-09-16T19:22:05+08:00">
							in 1 hour
						</button>
					</li>
					<li>
						<button type="button" class="btn-link dropdown-item ws-normal js-user-status-expire-button" title="in 4 hours" value="2020-09-16T22:22:05+08:00">
							in 4 hours
						</button>
					</li>
					<li>
						<button type="button" class="btn-link dropdown-item ws-normal js-user-status-expire-button" title="today" value="2020-09-16T23:59:59+08:00">
							today
						</button>
					</li>
					<li>
						<button type="button" class="btn-link dropdown-item ws-normal js-user-status-expire-button" title="this week" value="2020-09-20T23:59:59+08:00">
							this week
						</button>
					</li>
			</ul>
		</details>
		<input class="js-user-status-expiration-date-input" type="hidden" name="expires_at" value="">
	</div>
	
						<include-fragment class="js-user-status-org-picker" data-url="/users/status/organizations"></include-fragment>
					</div>
					<div class="d-flex flex-items-center flex-justify-between p-3 border-top">
						<button type="submit" disabled class="width-full btn btn-primary mr-2 js-user-status-submit">
							Set status
						</button>
						<button type="button" disabled class="width-full js-clear-user-status-button btn ml-2 ">
							Clear status
						</button>
					</div>
	</form>    </details-dialog>
		</details>
	</div>
	
				</div>
				<div role="none" class="dropdown-divider"></div>
	
			<a role="menuitem" class="dropdown-item" href="/fefit" data-ga-click="Header, go to profile, text:your profile" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;YOUR_PROFILE&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="9d5c04607b5dafc759e90db3b802c971490dae36e512d4db2ad24ab18d1e6b05" >Your profile</a>
	
			<a role="menuitem" class="dropdown-item" href="/fefit?tab=repositories" data-ga-click="Header, go to repositories, text:your repositories" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;YOUR_REPOSITORIES&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="862bbf80c0c88221e9407195cfe088c52728abd5f44ac32f73c61f7209688fc1" >Your repositories</a>
	
				<a role="menuitem"
					 class="dropdown-item"
					 href="/settings/organizations"
					 
					 data-ga-click="Header, go to organizations, text:your organizations"
					 data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;YOUR_ORGANIZATIONS&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="2605024914ad5712d6e7f8040182c01193bf67cde9c0f4582d87ab53c784f95e"
					 >Your organizations</a>
	
	
			<a role="menuitem" class="dropdown-item" href="/fefit?tab=projects" data-ga-click="Header, go to projects, text:your projects" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;YOUR_PROJECTS&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="c827f645759841a3fa160c8dbec801ef77721575ca30b63a975556fa9efa89f6" >Your projects</a>
	
	
			<a role="menuitem" class="dropdown-item" href="/fefit?tab=stars" data-ga-click="Header, go to starred repos, text:your stars" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;YOUR_STARS&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="db74974f0962df630789d9466c057299e79c46599b936cb71b06383cdfbae1b5" >Your stars</a>
				<a role="menuitem" class="dropdown-item" href="https://gist.github.com/mine" data-ga-click="Header, your gists, text:your gists" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;YOUR_GISTS&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="60282eb70bb753047b43e0d3e48ee43b7591e46c2f317bef7805c2eea6205b71" >Your gists</a>
	
	
	
	
	
			<div role="none" class="dropdown-divider"></div>
				<a role="menuitem" class="dropdown-item" href="/settings/billing" data-ga-click="Header, go to billing, text:upgrade" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;UPGRADE&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="e69d3abca4e374fb391e87946ba08f44f67f8791539f9178a414bbdf594fa60a" >Upgrade</a>
				
	<div id="feature-enrollment-toggle" class="hide-sm hide-md feature-preview-details position-relative">
		<button
			type="button"
			class="dropdown-item btn-link"
			role="menuitem"
			data-feature-preview-trigger-url="/users/fefit/feature_previews"
			data-feature-preview-close-details="{&quot;event_type&quot;:&quot;feature_preview.clicks.close_modal&quot;,&quot;payload&quot;:{&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}"
			data-feature-preview-close-hmac="bfdbc7c76d348ea9d9fd3914900ce6bdba0c49ee21243d308c9a97f7b9ed0247"
			data-hydro-click="{&quot;event_type&quot;:&quot;feature_preview.clicks.open_modal&quot;,&quot;payload&quot;:{&quot;link_location&quot;:&quot;user_dropdown&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}"
			data-hydro-click-hmac="91c3a81933495bb3d4d2315b66b9bd66dd89bc6647228a811c6444f2742804bd"
		>
			Feature preview
		</button>
			<span class="feature-preview-indicator js-feature-preview-indicator" hidden></span>
	</div>
	
			<a role="menuitem" class="dropdown-item" href="https://docs.github.com" data-ga-click="Header, go to help, text:help" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;HELP&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="eb2b0f1f2c3b86db3c80149b6534a469167764cd32c09ef675f680fd126d2acd" >Help</a>
			<a role="menuitem" class="dropdown-item" href="/settings/profile" data-ga-click="Header, go to settings, icon:settings" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;SETTINGS&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="d4efe6114bda336ff45db26303941a4f89bb3bbedc0f4b5535368a320f3c5c7d" >Settings</a>
			<!-- '"` --><!-- </textarea></xmp> --></option></form><form class="logout-form" action="/logout" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="67Fwp7crkONij54MexF0DYhCDzkSeve7L4tewab7DglknxGP72fk2JVzfQyRDVVipuotDimQ40J9O1RSv1pyCw==" />
				
				<button type="submit" class="dropdown-item dropdown-signout" data-ga-click="Header, sign out, icon:logout" data-hydro-click="{&quot;event_type&quot;:&quot;global_header.user_menu_dropdown.click&quot;,&quot;payload&quot;:{&quot;request_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;target&quot;:&quot;SIGN_OUT&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="df03de5fa0a6eac93f9a567a61fcb8b38e9a95e88baaf5a0fd9d2fa9f01e334f"  role="menuitem">
					Sign out
				</button>
				<input type="text" name="required_field_beb4" hidden="hidden" class="form-control" /><input type="hidden" name="timestamp" value="1600251725638" class="form-control" /><input type="hidden" name="timestamp_secret" value="9777c8079a67c392a0a72917a777f04901b7b95332b3cac54d0d3f48734fe4fd" class="form-control" />
	</form>  </details-menu>
	</details>
	
		</div>
	
	</header>
	
						
	
			</div>
	
		<div id="start-of-content" class="show-on-focus"></div>
	
	
	
	
			<div id="js-flash-container">
	
	
		<template class="js-flash-template">
			<div class="flash flash-full  {{ className }}">
		<div class=" px-2" >
			<button class="flash-close js-flash-close" type="button" aria-label="Dismiss this message">
				<svg class="octicon octicon-x" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path></svg>
			</button>
			
				<div>{{ message }}</div>
	
		</div>
	</div>
		</template>
	</div>
	
	
		
	
		<include-fragment class="js-notification-shelf-include-fragment" data-base-src="https://github.com/notifications/beta/shelf"></include-fragment>
	
	
	
		<div
			class="application-main "
			data-commit-hovercards-enabled
			data-discussion-hovercards-enabled
			data-issue-and-pr-hovercards-enabled
		>
					<div itemscope itemtype="http://schema.org/SoftwareSourceCode" class="">
			<main id="js-repo-pjax-container" data-pjax-container >
				
	
			
	
					<div class="border-bottom shelf intro-shelf js-notice mb-0 pb-4">
		<div class="width-full container">
			<div class="width-full mx-auto shelf-content">
				<h2 class="shelf-title">Learn Git and GitHub without any code!</h2>
				<p class="shelf-lead">
						Using the Hello World guide, you’ll start a branch, write comments, and open a pull request.
				</p>
				<a class="btn btn-primary shelf-cta" target="_blank" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;READ_GUIDE&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="881d46753126239be195640235585376e9fd5c97f7941de0a85a7851de33f09c" href="https://guides.github.com/activities/hello-world/">Read the guide</a>
			</div>
			<!-- '"` --><!-- </textarea></xmp> --></option></form><form class="shelf-dismiss js-notice-dismiss" action="/dashboard/dismiss_bootcamp" accept-charset="UTF-8" method="post"><input type="hidden" name="_method" value="delete" /><input type="hidden" name="authenticity_token" value="sEkBEtHzouOvmDN6y85FI379M6+svGaqze6OgH9Ihcq/LOxgoeoTxTR3MZyXjrPjvqGlLPqNKXR8+ES/xv4YkA==" />
				<button name="button" type="submit" class="mr-1 close-button tooltipped tooltipped-w" aria-label="Hide this notice forever" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;DISMISS_BANNER&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="0ff4a1cf0079c727336e372949d7614df350ae223971bff5c2ad0ab4e309b88e">
					<svg aria-label="Hide this notice forever" class="octicon octicon-x v-align-text-top" viewBox="0 0 16 16" version="1.1" width="16" height="16" role="img"><path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path></svg>
	</button></form>  </div>
	</div>
	
	
	
	
	
	
		
	
	
		<div class="bg-gray-light pt-3 hide-full-screen mb-5">
	
				<div class="d-flex mb-3 px-3 px-md-4 px-lg-5">
	
					<div class="flex-auto min-width-0 width-fit mr-3">
							<h1 class=" d-flex flex-wrap flex-items-center break-word f3 text-normal">
			<svg class="octicon octicon-repo text-gray" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
		<span class="author ml-2 flex-self-stretch" itemprop="author">
			<a class="url fn" rel="author" data-hovercard-type="user" data-hovercard-url="/users/bheisler/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/bheisler">bheisler</a>
		</span>
		<span class="mx-1 flex-self-stretch">/</span>
		<strong itemprop="name" class="mr-2 flex-self-stretch">
			<a data-pjax="#js-repo-pjax-container" href="/bheisler/criterion.rs">criterion.rs</a>
		</strong>
		
	</h1>
	
	
					</div>
	
						<ul class="pagehead-actions flex-shrink-0 d-none d-md-inline" style="padding: 2px 0;">
	
		<li>
							<form data-remote="true" class="d-flex js-social-form js-social-container" action="/notifications/subscribe" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="eot9jXQvCZXGgTYWusSW/cICfUJwhSWCCekGQmwvfgh1g/feNTZfB0yMgA0ngQsOU2S1R3E+95z7qJofxNEemA==" />      <input type="hidden" name="repository_id" value="20188259">
	
				<details class="details-reset details-overlay select-menu hx_rsm">
					<summary class="btn btn-sm btn-with-count" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;WATCH_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="197cc08bba03c4f8328f991aac2453918133af5f3b5ec192046355a8c61c8b3d" data-ga-click="Repository, click Watch settings, action:files#disambiguate">          <span data-menu-button>
								<svg height="16" class="octicon octicon-eye" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z"></path></svg>
								Watch
						</span>
						<span class="dropdown-caret"></span>
	</summary>        <details-menu
						class="select-menu-modal position-absolute mt-5"
						style="z-index: 99;">
						<div class="select-menu-header">
							<span class="select-menu-title">Notifications</span>
						</div>
						<div class="select-menu-list">
							<button type="submit" name="do" value="included" class="select-menu-item width-full" aria-checked="true" role="menuitemradio">
								<svg class="octicon octicon-check select-menu-item-icon" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path></svg>
								<div class="select-menu-item-text">
									<span class="select-menu-item-heading">Not watching</span>
									<span class="description">Be notified only when participating or @mentioned.</span>
									<span class="hidden-select-button-text" data-menu-button-contents>
										<svg height="16" class="octicon octicon-eye" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z"></path></svg>
										Watch
									</span>
								</div>
							</button>
	
							<button type="submit" name="do" value="release_only" class="select-menu-item width-full" aria-checked="false" role="menuitemradio">
								<svg class="octicon octicon-check select-menu-item-icon" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path></svg>
								<div class="select-menu-item-text">
									<span class="select-menu-item-heading">Releases only</span>
									<span class="description">Be notified of new releases, and when participating or @mentioned.</span>
									<span class="hidden-select-button-text" data-menu-button-contents>
										<svg height="16" class="octicon octicon-eye" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z"></path></svg>
										Unwatch releases
									</span>
								</div>
							</button>
	
							<button type="submit" name="do" value="subscribed" class="select-menu-item width-full" aria-checked="false" role="menuitemradio">
								<svg class="octicon octicon-check select-menu-item-icon" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path></svg>
								<div class="select-menu-item-text">
									<span class="select-menu-item-heading">Watching</span>
									<span class="description">Be notified of all conversations.</span>
									<span class="hidden-select-button-text" data-menu-button-contents>
										<svg class="octicon octicon-eye v-align-text-bottom" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z"></path></svg>
										Unwatch
									</span>
								</div>
							</button>
	
							<button type="submit" name="do" value="ignore" class="select-menu-item width-full" aria-checked="false" role="menuitemradio">
								<svg class="octicon octicon-check select-menu-item-icon" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path></svg>
								<div class="select-menu-item-text">
									<span class="select-menu-item-heading">Ignoring</span>
									<span class="description">Never be notified.</span>
									<span class="hidden-select-button-text" data-menu-button-contents>
										<svg height="16" class="octicon octicon-bell-slash" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 1.5c-.997 0-1.895.416-2.534 1.086A.75.75 0 014.38 1.55 5 5 0 0113 5v2.373a.75.75 0 01-1.5 0V5A3.5 3.5 0 008 1.5zM4.182 4.31L1.19 2.143a.75.75 0 10-.88 1.214L3 5.305v2.642a.25.25 0 01-.042.139L1.255 10.64A1.518 1.518 0 002.518 13h11.108l1.184.857a.75.75 0 10.88-1.214l-1.375-.996a1.196 1.196 0 00-.013-.01L4.198 4.321a.733.733 0 00-.016-.011zm7.373 7.19L4.5 6.391v1.556c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01.015.015 0 00.005.012.017.017 0 00.006.004l.007.001h9.037zM8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z"></path></svg>
										Stop ignoring
									</span>
								</div>
							</button>
						</div>
					</details-menu>
				</details>
					<a class="social-count js-social-count"
						href="/bheisler/criterion.rs/watchers"
						aria-label="18 users are watching this repository">
						18
					</a>
	</form>
		</li>
	
		<li>
					<div class="js-toggler-container js-social-container starring-container ">
			<form class="starred js-social-form" action="/bheisler/criterion.rs/unstar" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="OFL1EDPIRmrHzXTE4T57fJsu0qYOqyucHzRADWCxPxTLaZDUoe8Sbu3MpOGSk897r7dvEzHXI5MInB3/mHVqhw==" />
				<input type="hidden" name="context" value="repository"></input>
				<button type="submit" class="btn btn-sm btn-with-count  js-toggler-target" aria-label="Unstar this repository" title="Unstar bheisler/criterion.rs" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;UNSTAR_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="704d4d96e2f72c5588ca85a3a8eb8f8a10bc71e967370899f0c714ab53be18f7" data-ga-click="Repository, click unstar button, action:files#disambiguate; text:Unstar">        <svg height="16" class="octicon octicon-star-fill" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z"></path></svg>
					Unstar
	</button>        <a class="social-count js-social-count" href="/bheisler/criterion.rs/stargazers"
						 aria-label="1397 users starred this repository">
						 1.4k
					</a>
	</form>
			<form class="unstarred js-social-form" action="/bheisler/criterion.rs/star" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="n2oP2EketIxB35LwvPOwKMsUpsMgP7oVJfKzjPRRO1Q3TD8YRjFoBl4WfJqT0KV5ygFW74Cff4/5OJeohGBYpQ==" />
				<input type="hidden" name="context" value="repository"></input>
				<button type="submit" class="btn btn-sm btn-with-count  js-toggler-target" aria-label="Unstar this repository" title="Star bheisler/criterion.rs" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;STAR_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="73ffe2b73a68d6cd44e1eb49f18a50e6b821d6271b86a5469397d433a19976f0" data-ga-click="Repository, click star button, action:files#disambiguate; text:Star">        <svg height="16" class="octicon octicon-star" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z"></path></svg>
					Star
	</button>        <a class="social-count js-social-count" href="/bheisler/criterion.rs/stargazers"
						 aria-label="1397 users starred this repository">
						1.4k
					</a>
	</form>  </div>
	
		</li>
	
		<li>
							<div class="float-left">
								<details class="details-reset details-overlay details-overlay-dark " >
												<summary
											class="btn btn-sm btn-with-count"
											title="Fork your own copy of bheisler/criterion.rs to your account"
											data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;FORK_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="a54ca5363963c0e939ba1f754e54370d211e946e1df1c74483d33c9b12b34bab" data-ga-click="Repository, show fork modal, action:files#disambiguate; text:Fork">
											<svg class="octicon octicon-repo-forked" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z"></path></svg>
											Fork
										</summary>
	
		<details-dialog
			class="Box d-flex flex-column anim-fade-in fast Box--overlay"
				aria-label="Fork criterion.rs"
				src="/bheisler/criterion.rs/fork?fragment=1"
				preload
			>
			<div class="Box-header">
				<button class="Box-btn-octicon btn-octicon float-right" type="button" aria-label="Close dialog" data-close-dialog>
					<svg height="16" class="octicon octicon-x" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path></svg>
				</button>
				<h1 class="Box-title">Fork criterion.rs</h1>
			</div>
				
									<div class="text-center overflow-auto">
										<include-fragment>
											<div class="octocat-spinner my-5" aria-label="Loading..."></div>
											<p class="f5 text-gray">If this dialog fails to load, you can visit <a href="/bheisler/criterion.rs/fork">the fork page</a> directly.</p>
										</include-fragment>
									</div>
	
		</details-dialog>
	</details>
							</div>
	
				<a href="/bheisler/criterion.rs/network/members" class="social-count"
					 aria-label="121 users forked this repository">
					121
				</a>
		</li>
	</ul>
	
				</div>
						<div class="d-block d-md-none mb-2 px-3 px-md-4 px-lg-5">
				<p class="f4 mb-3">
					Statistics-driven benchmarking library for Rust
				</p>
				<div class="mb-2">
					<a href="/bheisler/criterion.rs/blob/master/LICENSE-APACHE" class="muted-link">
						<svg mr="1" height="16" class="octicon octicon-law mr-1" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8.75.75a.75.75 0 00-1.5 0V2h-.984c-.305 0-.604.08-.869.23l-1.288.737A.25.25 0 013.984 3H1.75a.75.75 0 000 1.5h.428L.066 9.192a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.514 3.514 0 00.686.45A4.492 4.492 0 003 11c.88 0 1.556-.22 2.023-.454a3.515 3.515 0 00.686-.45l.045-.04.016-.015.006-.006.002-.002.001-.002L5.25 9.5l.53.53a.75.75 0 00.154-.838L3.822 4.5h.162c.305 0 .604-.08.869-.23l1.289-.737a.25.25 0 01.124-.033h.984V13h-2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-2.5V3.5h.984a.25.25 0 01.124.033l1.29.736c.264.152.563.231.868.231h.162l-2.112 4.692a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.517 3.517 0 00.686.45A4.492 4.492 0 0013 11c.88 0 1.556-.22 2.023-.454a3.512 3.512 0 00.686-.45l.045-.04.01-.01.006-.005.006-.006.002-.002.001-.002-.529-.531.53.53a.75.75 0 00.154-.838L13.823 4.5h.427a.75.75 0 000-1.5h-2.234a.25.25 0 01-.124-.033l-1.29-.736A1.75 1.75 0 009.735 2H8.75V.75zM1.695 9.227c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L3 6.327l-1.305 2.9zm10 0c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L13 6.327l-1.305 2.9z"></path></svg>
							View license
					</a>
				</div>
			<div class="mb-3">
				<a class="link-gray no-underline mr-3" href="/bheisler/criterion.rs/stargazers">
					<svg mr="1" height="16" class="octicon octicon-star mr-1" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z"></path></svg>
					<span class="text-bold">1.4k</span>
					stars
	</a>      <a class="link-gray no-underline" href="/bheisler/criterion.rs/network/members">
					<svg mr="1" height="16" class="octicon octicon-repo-forked mr-1" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z"></path></svg>
					<span class="text-bold">121</span>
					forks
	</a>    </div>
			<div class="d-flex">
				<div class="flex-1 mr-2">
						<div class="js-toggler-container js-social-container starring-container ">
			<form class="starred js-social-form" action="/bheisler/criterion.rs/unstar" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="Hlxh3M49GH3EnlYuaQvl2mwbkUT3cXLPoU2F8O8g2vPtZwQYXBpMee6fhgsaplHdWIIs8cgNesC25dgCF+SPYA==" />
				<input type="hidden" name="context" value="repository"></input>
				<button type="submit" class="btn btn-sm  btn-block js-toggler-target" aria-label="Unstar this repository" title="Unstar bheisler/criterion.rs" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;UNSTAR_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="704d4d96e2f72c5588ca85a3a8eb8f8a10bc71e967370899f0c714ab53be18f7" data-ga-click="Repository, click unstar button, action:files#disambiguate; text:Unstar">        <svg height="16" class="octicon octicon-star-fill" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z"></path></svg>
					Unstar
	</button></form>
			<form class="unstarred js-social-form" action="/bheisler/criterion.rs/star" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="3403mY7psQ3SRR8cJyGmo6WCR/HefB01BzYRyzRW1rx3qwdZgcZth82M8XYIArPypJe33X7c2K/b/DXvRGe1TQ==" />
				<input type="hidden" name="context" value="repository"></input>
				<button type="submit" class="btn btn-sm  btn-block js-toggler-target" aria-label="Unstar this repository" title="Star bheisler/criterion.rs" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;STAR_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="73ffe2b73a68d6cd44e1eb49f18a50e6b821d6271b86a5469397d433a19976f0" data-ga-click="Repository, click star button, action:files#disambiguate; text:Star">        <svg height="16" class="octicon octicon-star" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z"></path></svg>
					Star
	</button></form>  </div>
	
				</div>
				<div class="flex-1">
							<form data-remote="true" class=" js-social-form js-social-container" action="/notifications/subscribe" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="hGtjpZNpKgoh3bt45T4Q4mB447VRy6hAP6Ab8mX1VyyLY+n20nB8mKvQDWN4e40R8R4rsFBwel7N4YevzQs3vA==" />      <input type="hidden" name="repository_id" value="20188259">
	
				<details class="details-reset details-overlay select-menu hx_rsm">
					<summary class="btn btn-sm btn-block" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;WATCH_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="197cc08bba03c4f8328f991aac2453918133af5f3b5ec192046355a8c61c8b3d" data-ga-click="Repository, click Watch settings, action:files#disambiguate">          <span data-menu-button>
								<svg height="16" class="octicon octicon-eye" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z"></path></svg>
								Watch
						</span>
						<span class="dropdown-caret"></span>
	</summary>        <details-menu
						class="select-menu-modal position-absolute mt-5"
						style="z-index: 99;">
						<div class="select-menu-header">
							<span class="select-menu-title">Notifications</span>
						</div>
						<div class="select-menu-list">
							<button type="submit" name="do" value="included" class="select-menu-item width-full" aria-checked="true" role="menuitemradio">
								<svg class="octicon octicon-check select-menu-item-icon" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path></svg>
								<div class="select-menu-item-text">
									<span class="select-menu-item-heading">Not watching</span>
									<span class="description">Be notified only when participating or @mentioned.</span>
									<span class="hidden-select-button-text" data-menu-button-contents>
										<svg height="16" class="octicon octicon-eye" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z"></path></svg>
										Watch
									</span>
								</div>
							</button>
	
							<button type="submit" name="do" value="release_only" class="select-menu-item width-full" aria-checked="false" role="menuitemradio">
								<svg class="octicon octicon-check select-menu-item-icon" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path></svg>
								<div class="select-menu-item-text">
									<span class="select-menu-item-heading">Releases only</span>
									<span class="description">Be notified of new releases, and when participating or @mentioned.</span>
									<span class="hidden-select-button-text" data-menu-button-contents>
										<svg height="16" class="octicon octicon-eye" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z"></path></svg>
										Unwatch releases
									</span>
								</div>
							</button>
	
							<button type="submit" name="do" value="subscribed" class="select-menu-item width-full" aria-checked="false" role="menuitemradio">
								<svg class="octicon octicon-check select-menu-item-icon" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path></svg>
								<div class="select-menu-item-text">
									<span class="select-menu-item-heading">Watching</span>
									<span class="description">Be notified of all conversations.</span>
									<span class="hidden-select-button-text" data-menu-button-contents>
										<svg class="octicon octicon-eye v-align-text-bottom" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z"></path></svg>
										Unwatch
									</span>
								</div>
							</button>
	
							<button type="submit" name="do" value="ignore" class="select-menu-item width-full" aria-checked="false" role="menuitemradio">
								<svg class="octicon octicon-check select-menu-item-icon" height="16" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path></svg>
								<div class="select-menu-item-text">
									<span class="select-menu-item-heading">Ignoring</span>
									<span class="description">Never be notified.</span>
									<span class="hidden-select-button-text" data-menu-button-contents>
										<svg height="16" class="octicon octicon-bell-slash" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 1.5c-.997 0-1.895.416-2.534 1.086A.75.75 0 014.38 1.55 5 5 0 0113 5v2.373a.75.75 0 01-1.5 0V5A3.5 3.5 0 008 1.5zM4.182 4.31L1.19 2.143a.75.75 0 10-.88 1.214L3 5.305v2.642a.25.25 0 01-.042.139L1.255 10.64A1.518 1.518 0 002.518 13h11.108l1.184.857a.75.75 0 10.88-1.214l-1.375-.996a1.196 1.196 0 00-.013-.01L4.198 4.321a.733.733 0 00-.016-.011zm7.373 7.19L4.5 6.391v1.556c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01.015.015 0 00.005.012.017.017 0 00.006.004l.007.001h9.037zM8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z"></path></svg>
										Stop ignoring
									</span>
								</div>
							</button>
						</div>
					</details-menu>
				</details>
	</form>
				</div>
			</div>
		</div>
	
					
	<nav aria-label="Repository" data-pjax="#js-repo-pjax-container" class="js-repo-nav js-sidenav-container-pjax js-responsive-underlinenav overflow-hidden UnderlineNav px-3 px-md-4 px-lg-5 bg-gray-light">
		<ul class="UnderlineNav-body list-style-none ">
						<li class="d-flex">
					<a class="js-selected-navigation-item selected UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item" data-tab-item="code-tab" data-hotkey="g c" data-ga-click="Repository, Navigation click, Code tab" aria-current="page" data-selected-links="repo_source repo_downloads repo_commits repo_releases repo_tags repo_branches repo_packages repo_deployments /bheisler/criterion.rs" href="/bheisler/criterion.rs">
								<svg classes="UnderlineNav-octicon" display="none inline" height="16" class="octicon octicon-code UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M4.72 3.22a.75.75 0 011.06 1.06L2.06 8l3.72 3.72a.75.75 0 11-1.06 1.06L.47 8.53a.75.75 0 010-1.06l4.25-4.25zm6.56 0a.75.75 0 10-1.06 1.06L13.94 8l-3.72 3.72a.75.75 0 101.06 1.06l4.25-4.25a.75.75 0 000-1.06l-4.25-4.25z"></path></svg>
							<span data-content="Code">Code</span>
								<span title="Not available" class="Counter "></span>
	</a>      </li>
				<li class="d-flex">
					<a class="js-selected-navigation-item UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item" data-tab-item="issues-tab" data-hotkey="g i" data-ga-click="Repository, Navigation click, Issues tab" data-selected-links="repo_issues repo_labels repo_milestones /bheisler/criterion.rs/issues" href="/bheisler/criterion.rs/issues">
								<svg classes="UnderlineNav-octicon" display="none inline" height="16" class="octicon octicon-issue-opened UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm9 3a1 1 0 11-2 0 1 1 0 012 0zm-.25-6.25a.75.75 0 00-1.5 0v3.5a.75.75 0 001.5 0v-3.5z"></path></svg>
							<span data-content="Issues">Issues</span>
								<span title="42" class="Counter ">42</span>
	</a>      </li>
				<li class="d-flex">
					<a class="js-selected-navigation-item UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item" data-tab-item="pull-requests-tab" data-hotkey="g p" data-ga-click="Repository, Navigation click, Pull requests tab" data-selected-links="repo_pulls checks /bheisler/criterion.rs/pulls" href="/bheisler/criterion.rs/pulls">
								<svg classes="UnderlineNav-octicon" display="none inline" height="16" class="octicon octicon-git-pull-request UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.177 3.073L9.573.677A.25.25 0 0110 .854v4.792a.25.25 0 01-.427.177L7.177 3.427a.25.25 0 010-.354zM3.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122v5.256a2.251 2.251 0 11-1.5 0V5.372A2.25 2.25 0 011.5 3.25zM11 2.5h-1V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5zm1 10.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.75 12a.75.75 0 100 1.5.75.75 0 000-1.5z"></path></svg>
							<span data-content="Pull requests">Pull requests</span>
								<span title="8" class="Counter ">8</span>
	</a>      </li>
				<li class="d-flex">
					<a class="js-selected-navigation-item UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item" data-tab-item="actions-tab" data-hotkey="g a" data-ga-click="Repository, Navigation click, Actions tab" data-selected-links="repo_actions /bheisler/criterion.rs/actions" href="/bheisler/criterion.rs/actions">
								<svg classes="UnderlineNav-octicon" display="none inline" height="16" class="octicon octicon-play UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zM6.379 5.227A.25.25 0 006 5.442v5.117a.25.25 0 00.379.214l4.264-2.559a.25.25 0 000-.428L6.379 5.227z"></path></svg>
							<span data-content="Actions">Actions</span>
								<span title="Not available" class="Counter "></span>
	</a>      </li>
				<li class="d-flex">
					<a class="js-selected-navigation-item UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item" data-tab-item="wiki-tab" data-hotkey="g w" data-ga-click="Repository, Navigation click, Wikis tab" data-selected-links="repo_wiki /bheisler/criterion.rs/wiki" href="/bheisler/criterion.rs/wiki">
								<svg classes="UnderlineNav-octicon" display="none inline" height="16" class="octicon octicon-book UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M0 1.75A.75.75 0 01.75 1h4.253c1.227 0 2.317.59 3 1.501A3.744 3.744 0 0111.006 1h4.245a.75.75 0 01.75.75v10.5a.75.75 0 01-.75.75h-4.507a2.25 2.25 0 00-1.591.659l-.622.621a.75.75 0 01-1.06 0l-.622-.621A2.25 2.25 0 005.258 13H.75a.75.75 0 01-.75-.75V1.75zm8.755 3a2.25 2.25 0 012.25-2.25H14.5v9h-3.757c-.71 0-1.4.201-1.992.572l.004-7.322zm-1.504 7.324l.004-5.073-.002-2.253A2.25 2.25 0 005.003 2.5H1.5v9h3.757a3.75 3.75 0 011.994.574z"></path></svg>
							<span data-content="Wiki">Wiki</span>
								<span title="Not available" class="Counter "></span>
	</a>      </li>
				<li class="d-flex">
					<a class="js-selected-navigation-item UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item" data-tab-item="security-tab" data-hotkey="g s" data-ga-click="Repository, Navigation click, Security tab" data-selected-links="security overview alerts policy token_scanning code_scanning /bheisler/criterion.rs/security" href="/bheisler/criterion.rs/security">
								<svg classes="UnderlineNav-octicon" display="none inline" height="16" class="octicon octicon-shield UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.467.133a1.75 1.75 0 011.066 0l5.25 1.68A1.75 1.75 0 0115 3.48V7c0 1.566-.32 3.182-1.303 4.682-.983 1.498-2.585 2.813-5.032 3.855a1.7 1.7 0 01-1.33 0c-2.447-1.042-4.049-2.357-5.032-3.855C1.32 10.182 1 8.566 1 7V3.48a1.75 1.75 0 011.217-1.667l5.25-1.68zm.61 1.429a.25.25 0 00-.153 0l-5.25 1.68a.25.25 0 00-.174.238V7c0 1.358.275 2.666 1.057 3.86.784 1.194 2.121 2.34 4.366 3.297a.2.2 0 00.154 0c2.245-.956 3.582-2.104 4.366-3.298C13.225 9.666 13.5 8.36 13.5 7V3.48a.25.25 0 00-.174-.237l-5.25-1.68zM9 10.5a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.75a.75.75 0 10-1.5 0v3a.75.75 0 001.5 0v-3z"></path></svg>
							<span data-content="Security">Security</span>
								<include-fragment src="/bheisler/criterion.rs/security/overall-count" accept="text/fragment+html"></include-fragment>
	</a>      </li>
				<li class="d-flex">
					<a class="js-selected-navigation-item UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item" data-tab-item="insights-tab" data-ga-click="Repository, Navigation click, Insights tab" data-selected-links="repo_graphs repo_contributors dependency_graph dependabot_updates pulse people /bheisler/criterion.rs/pulse" href="/bheisler/criterion.rs/pulse">
								<svg classes="UnderlineNav-octicon" display="none inline" height="16" class="octicon octicon-graph UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.5 1.75a.75.75 0 00-1.5 0v12.5c0 .414.336.75.75.75h14.5a.75.75 0 000-1.5H1.5V1.75zm14.28 2.53a.75.75 0 00-1.06-1.06L10 7.94 7.53 5.47a.75.75 0 00-1.06 0L3.22 8.72a.75.75 0 001.06 1.06L7 7.06l2.47 2.47a.75.75 0 001.06 0l5.25-5.25z"></path></svg>
							<span data-content="Insights">Insights</span>
								<span title="Not available" class="Counter "></span>
	</a>      </li>
	
	</ul>        <div class="position-absolute right-0 pr-3 pr-md-4 pr-lg-5 js-responsive-underlinenav-overflow" style="visibility:hidden;">
				<details class="details-overlay details-reset position-relative">
		<summary role="button">
			<div class="UnderlineNav-item mr-0 border-0">
							<svg class="octicon octicon-kebab-horizontal" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M8 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zM1.5 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zm13 0a1.5 1.5 0 100-3 1.5 1.5 0 000 3z"></path></svg>
							<span class="sr-only">More</span>
						</div>
	</summary>  <div>
			<details-menu role="menu" class="dropdown-menu dropdown-menu-sw ">
		
							<ul>
									<li data-menu-item="code-tab" hidden>
										<a role="menuitem" class="js-selected-navigation-item selected dropdown-item" aria-current="page" data-selected-links=" /bheisler/criterion.rs" href="/bheisler/criterion.rs">
											Code
	</a>                </li>
									<li data-menu-item="issues-tab" hidden>
										<a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bheisler/criterion.rs/issues" href="/bheisler/criterion.rs/issues">
											Issues
	</a>                </li>
									<li data-menu-item="pull-requests-tab" hidden>
										<a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bheisler/criterion.rs/pulls" href="/bheisler/criterion.rs/pulls">
											Pull requests
	</a>                </li>
									<li data-menu-item="actions-tab" hidden>
										<a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bheisler/criterion.rs/actions" href="/bheisler/criterion.rs/actions">
											Actions
	</a>                </li>
									<li data-menu-item="wiki-tab" hidden>
										<a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bheisler/criterion.rs/wiki" href="/bheisler/criterion.rs/wiki">
											Wiki
	</a>                </li>
									<li data-menu-item="security-tab" hidden>
										<a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bheisler/criterion.rs/security" href="/bheisler/criterion.rs/security">
											Security
	</a>                </li>
									<li data-menu-item="insights-tab" hidden>
										<a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bheisler/criterion.rs/pulse" href="/bheisler/criterion.rs/pulse">
											Insights
	</a>                </li>
							</ul>
	
	</details-menu>
	</div></details>    </div>
	
	</nav>
		</div>
	
	<div class="container-xl clearfix new-discussion-timeline  px-3 px-md-4 px-lg-5">
		<div class="repository-content " >
	
			
				
	
		<div class="d-none d-lg-block mt-6 mr-3 Popover top-0 right-0 box-shadow-medium col-3">
			
		</div>
	
		
	
	
		<div class="gutter-condensed gutter-lg flex-column flex-md-row d-flex">
	
		<div class="flex-shrink-0 col-12 col-md-9 mb-4 mb-md-0">
			
	
	
	
					<include-fragment src="/bheisler/criterion.rs/show_partial?partial=tree%2Frecently_touched_branches_list"></include-fragment>
	
				<div class="file-navigation mb-3 d-flex flex-items-start">
		
	<div class="position-relative">
		<details class="details-reset details-overlay mr-0 mb-0 " id="branch-select-menu">
			<summary class="btn css-truncate"
							data-hotkey="w"
							title="Switch branches or tags">
				<svg text="gray" height="16" class="octicon octicon-git-branch text-gray" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z"></path></svg>
				<span class="css-truncate-target" data-menu-button>master</span>
				<span class="dropdown-caret"></span>
			</summary>
	
			<details-menu class="SelectMenu SelectMenu--hasFilter" src="/bheisler/criterion.rs/refs/master?source_action=disambiguate&amp;source_controller=files" preload>
				<div class="SelectMenu-modal">
					<include-fragment class="SelectMenu-loading" aria-label="Menu is loading">
						<svg class="octicon octicon-octoface anim-pulse" height="32" viewBox="0 0 24 24" version="1.1" width="32" aria-hidden="true"><path d="M7.75 11c-.69 0-1.25.56-1.25 1.25v1.5a1.25 1.25 0 102.5 0v-1.5C9 11.56 8.44 11 7.75 11zm1.27 4.5a.469.469 0 01.48-.5h5a.47.47 0 01.48.5c-.116 1.316-.759 2.5-2.98 2.5s-2.864-1.184-2.98-2.5zm7.23-4.5c-.69 0-1.25.56-1.25 1.25v1.5a1.25 1.25 0 102.5 0v-1.5c0-.69-.56-1.25-1.25-1.25z"></path><path fill-rule="evenodd" d="M21.255 3.82a1.725 1.725 0 00-2.141-1.195c-.557.16-1.406.44-2.264.866-.78.386-1.647.93-2.293 1.677A18.442 18.442 0 0012 5c-.93 0-1.784.059-2.569.17-.645-.74-1.505-1.28-2.28-1.664a13.876 13.876 0 00-2.265-.866 1.725 1.725 0 00-2.141 1.196 23.645 23.645 0 00-.69 3.292c-.125.97-.191 2.07-.066 3.112C1.254 11.882 1 13.734 1 15.527 1 19.915 3.13 23 12 23c8.87 0 11-3.053 11-7.473 0-1.794-.255-3.647-.99-5.29.127-1.046.06-2.15-.066-3.125a23.652 23.652 0 00-.689-3.292zM20.5 14c.5 3.5-1.5 6.5-8.5 6.5s-9-3-8.5-6.5c.583-4 3-6 8.5-6s7.928 2 8.5 6z"></path></svg>
					</include-fragment>
				</div>
			</details-menu>
		</details>
	
	</div>
	
	
		<div class="flex-self-center ml-3 flex-self-stretch d-none d-lg-flex flex-items-center lh-condensed-ultra">
			<a data-pjax href="/bheisler/criterion.rs/branches" class="link-gray-dark no-underline">
				<svg text="gray" height="16" class="octicon octicon-git-branch text-gray" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z"></path></svg>
				<strong>8</strong>
				<span class="text-gray-light">branches</span>
			</a>
			<a data-pjax href="/bheisler/criterion.rs/tags" class="ml-3 link-gray-dark no-underline">
				<svg text="gray" height="16" class="octicon octicon-tag text-gray" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z"></path></svg>
				<strong>19</strong>
				<span class="text-gray-light">tags</span>
			</a>
		</div>
	
		<div class="flex-auto"></div>
	
		<a class="btn ml-2 d-none d-md-block" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;FIND_FILE_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="bc7896301306981546d30e593137e83e5d5135d685a7bce7dfa8c8cd2d391e96" data-ga-click="Repository, find file, location:repo overview" data-hotkey="t" data-pjax="true" href="/bheisler/criterion.rs/find/master">
			Go to file
	</a>
			<details class="details-overlay details-reset position-relative d-block">
		<summary role="button" type="button" class="btn ml-2">
			<span class="d-none d-md-flex flex-items-center">
						Add file
						<span class="dropdown-caret ml-1"></span>
					</span>
					<span class="d-inline-block d-md-none">
						<svg aria-label="More options" height="16" class="octicon octicon-kebab-horizontal" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path d="M8 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zM1.5 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zm13 0a1.5 1.5 0 100-3 1.5 1.5 0 000 3z"></path></svg>
					</span>
	</summary>  <div>
			<ul class="dropdown-menu dropdown-menu-sw">
						<li class="d-block d-md-none">
							<a class="dropdown-item" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;FIND_FILE_BUTTON&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="bc7896301306981546d30e593137e83e5d5135d685a7bce7dfa8c8cd2d391e96" data-ga-click="Repository, find file, location:repo overview" data-hotkey="t" data-pjax="true" href="/bheisler/criterion.rs/find/master">
								Go to file
	</a>          </li>
							<li class="d-block d-md-none dropdown-divider" role="none"></li>
								<li><!-- '"` --><!-- </textarea></xmp> --></option></form><form action="/bheisler/criterion.rs/new/master" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="3niF7OGY81rUhPu8ffRoE2HKyVrGfVfpWYaJSAMT/ohF2zlD9JiaGVH6R7xcoHGe1+nEGUZIO+b0frkttaE8tg==" />
			<button class="dropdown-item btn-link" type="submit">
				Create new file
			</button>
		</form></li>
	
								<li><a href="/bheisler/criterion.rs/upload/master" class="dropdown-item">
			Upload files
		</a></li>
	
					</ul>
	</div></details>
	
	
			<span class="d-none d-md-flex ml-2">
					
	<get-repo>
		<details class="position-relative details-overlay details-reset" data-action="toggle:get-repo#onDetailsToggle">
			<summary class="btn btn-primary" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;repository_id&quot;:20188259,&quot;target&quot;:&quot;CLONE_OR_DOWNLOAD_BUTTON&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="5ebcd63e7ffc40f055ef0ca9a5ed8b208247dad022961a89a38e4a5163d20e8a">
				<svg class="octicon octicon-download mr-1" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.47 10.78a.75.75 0 001.06 0l3.75-3.75a.75.75 0 00-1.06-1.06L8.75 8.44V1.75a.75.75 0 00-1.5 0v6.69L4.78 5.97a.75.75 0 00-1.06 1.06l3.75 3.75zM3.75 13a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5z"></path></svg>
				Code
				<span class="dropdown-caret"></span>
	</summary>    <div class="position-relative">
				<div class="dropdown-menu dropdown-menu-sw p-0" style="top:6px;width:352px;">
					<div data-target="get-repo.modal">
						<div class="border-bottom p-3">
								<div class="get-repo-modal js-toggler-container on">
			<div class="https-clone-options">
				<!-- '"` --><!-- </textarea></xmp> --></option></form><form data-remote="true" action="/users/set_protocol?protocol_selector=ssh&amp;protocol_type=clone" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="RuFMZUjdLOS9lSyejPY7B1w/V5Xvdw6U8GzI4Xtlrl7B9SVOWTOifLlWYEIzuXUed/fplW6dJPkLCkPIXJXGfw==" /><button name="button" type="submit" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;USE_SSH&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="47e4617c752b754a5a1071b89e924aeb29a4dbe8bca6b5bb75a21971d20ee80c" class="btn-link f6 js-toggler-target float-right">Use SSH</button></form>
	
			<h4 class="mb-1">
				Clone with HTTPS
				<a class="muted-link" href="https://docs.github.com/articles/which-remote-url-should-i-use" target="_blank" title="Which remote URL should I use?">
					<svg class="octicon octicon-question" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm9 3a1 1 0 11-2 0 1 1 0 012 0zM6.92 6.085c.081-.16.19-.299.34-.398.145-.097.371-.187.74-.187.28 0 .553.087.738.225A.613.613 0 019 6.25c0 .177-.04.264-.077.318a.956.956 0 01-.277.245c-.076.051-.158.1-.258.161l-.007.004a7.728 7.728 0 00-.313.195 2.416 2.416 0 00-.692.661.75.75 0 001.248.832.956.956 0 01.276-.245 6.3 6.3 0 01.26-.16l.006-.004c.093-.057.204-.123.313-.195.222-.149.487-.355.692-.662.214-.32.329-.702.329-1.15 0-.76-.36-1.348-.863-1.725A2.76 2.76 0 008 4c-.631 0-1.155.16-1.572.438-.413.276-.68.638-.849.977a.75.75 0 101.342.67z"></path></svg>
				</a>
			</h4>
			<p class="mb-2 f5">
				Use Git or checkout with SVN using the web URL.
			</p>
	
			<div class="input-group">
		<input type="text" class="form-control input-monospace input-sm bg-gray-light" data-autoselect value="https://github.com/bheisler/criterion.rs.git" aria-label="https://github.com/bheisler/criterion.rs.git" readonly>
		<div class="input-group-button">
			<clipboard-copy value="https://github.com/bheisler/criterion.rs.git" aria-label="Copy to clipboard" class="btn btn-sm" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;COPY_URL&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="c41b21a5d3bd0c4c821ab669e37874f88ec3fb603a0a6592d363a3fdcd30baa3"><svg class="octicon octicon-clippy" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z"></path></svg></clipboard-copy>
		</div>
	</div>
	
		</div>
	
		<div class="ssh-clone-options">
				<!-- '"` --><!-- </textarea></xmp> --></option></form><form data-remote="true" action="/users/set_protocol?protocol_selector=https&amp;protocol_type=clone" accept-charset="UTF-8" method="post"><input type="hidden" name="authenticity_token" value="XZmkPPM2vy/U8gTVnf8k5sz2SNfgxDai9gZWMciffVzajc0X4tgxt9AxSAkisGr/5z7212EuHM8NYN0Y728VfQ==" /><button name="button" type="submit" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;USE_HTTPS&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="4a2ec59e358c169ed5f0b6a9588a86e39da17265b2d3a7f605990e78b5f92d43" class="btn-link f6 js-toggler-target float-right">Use HTTPS</button></form>
				<h4 class="mb-1">
					Clone with SSH
					<a class="muted-link" href="https://docs.github.com/articles/which-remote-url-should-i-use" target="_blank" title="Which remote URL should I use?">
						<svg class="octicon octicon-question" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm9 3a1 1 0 11-2 0 1 1 0 012 0zM6.92 6.085c.081-.16.19-.299.34-.398.145-.097.371-.187.74-.187.28 0 .553.087.738.225A.613.613 0 019 6.25c0 .177-.04.264-.077.318a.956.956 0 01-.277.245c-.076.051-.158.1-.258.161l-.007.004a7.728 7.728 0 00-.313.195 2.416 2.416 0 00-.692.661.75.75 0 001.248.832.956.956 0 01.276-.245 6.3 6.3 0 01.26-.16l.006-.004c.093-.057.204-.123.313-.195.222-.149.487-.355.692-.662.214-.32.329-.702.329-1.15 0-.76-.36-1.348-.863-1.725A2.76 2.76 0 008 4c-.631 0-1.155.16-1.572.438-.413.276-.68.638-.849.977a.75.75 0 101.342.67z"></path></svg>
					</a>
				</h4>
	
	
				<p class="mb-2 f5">
						Use a password protected SSH key.
				</p>
	
				<div class="input-group">
		<input type="text" class="form-control input-monospace input-sm bg-gray-light" data-autoselect value="git@github.com:bheisler/criterion.rs.git" aria-label="git@github.com:bheisler/criterion.rs.git" readonly>
		<div class="input-group-button">
			<clipboard-copy value="git@github.com:bheisler/criterion.rs.git" aria-label="Copy to clipboard" class="btn btn-sm" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;COPY_URL&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="c41b21a5d3bd0c4c821ab669e37874f88ec3fb603a0a6592d363a3fdcd30baa3"><svg class="octicon octicon-clippy" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z"></path></svg></clipboard-copy>
		</div>
	</div>
	
		</div>
	</div>
						</div>
						<ul class="list-style-none">
							<li data-platforms="windows,mac" class="Box-row Box-row--hover-gray p-0 rounded-0 mt-0 js-remove-unless-platform">
								<a class="d-flex flex-items-center text-gray-dark text-bold no-underline p-3" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;OPEN_IN_DESKTOP&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="44816701a1276906d4617f4415bdde7d8d40af685678498f30a831b0e6a0e0e2" data-action="click:get-repo#showDownloadMessage" href="https://desktop.github.com">
									<svg class="octicon octicon-desktop-download mr-3" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8.75 5V.75a.75.75 0 00-1.5 0V5H5.104a.25.25 0 00-.177.427l2.896 2.896a.25.25 0 00.354 0l2.896-2.896A.25.25 0 0010.896 5H8.75zM1.5 2.75a.25.25 0 01.25-.25h3a.75.75 0 000-1.5h-3A1.75 1.75 0 000 2.75v7.5C0 11.216.784 12 1.75 12h3.727c-.1 1.041-.52 1.872-1.292 2.757A.75.75 0 004.75 16h6.5a.75.75 0 00.565-1.243c-.772-.885-1.193-1.716-1.292-2.757h3.727A1.75 1.75 0 0016 10.25v-7.5A1.75 1.75 0 0014.25 1h-3a.75.75 0 000 1.5h3a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-7.5zM9.018 12H6.982a5.72 5.72 0 01-.765 2.5h3.566a5.72 5.72 0 01-.765-2.5z"></path></svg>
									Open with GitHub Desktop
	</a>            </li>
							<li class="Box-row Box-row--hover-gray p-0">
								<a class="d-flex flex-items-center text-gray-dark text-bold no-underline p-3" rel="nofollow" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;DOWNLOAD_ZIP&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:20188259,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}" data-hydro-click-hmac="687176a9f0da04bf54ca2b5ccc2d5ea25dbd7f4686fa8fcf3c7349f9d2812e7e" data-ga-click="Repository, download zip, location:repo overview" data-open-app="link" href="/bheisler/criterion.rs/archive/master.zip">
									<svg class="octicon octicon-file-zip mr-3" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M3.5 1.75a.25.25 0 01.25-.25h3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h2.086a.25.25 0 01.177.073l2.914 2.914a.25.25 0 01.073.177v8.586a.25.25 0 01-.25.25h-.5a.75.75 0 000 1.5h.5A1.75 1.75 0 0014 13.25V4.664c0-.464-.184-.909-.513-1.237L10.573.513A1.75 1.75 0 009.336 0H3.75A1.75 1.75 0 002 1.75v11.5c0 .649.353 1.214.874 1.515a.75.75 0 10.752-1.298.25.25 0 01-.126-.217V1.75zM8.75 3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM6 5.25a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5A.75.75 0 016 5.25zm2 1.5A.75.75 0 018.75 6h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 6.75zm-1.25.75a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM8 9.75A.75.75 0 018.75 9h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 9.75zm-.75.75a1.75 1.75 0 00-1.75 1.75v3c0 .414.336.75.75.75h2.5a.75.75 0 00.75-.75v-3a1.75 1.75 0 00-1.75-1.75h-.5zM7 12.25a.25.25 0 01.25-.25h.5a.25.25 0 01.25.25v2.25H7v-2.25z"></path></svg>
									Download ZIP
	</a>            </li>
						</ul>
					</div>
	
					<div class="p-3" data-targets="get-repo.platforms" data-platform="mac" hidden>
						<h4 class="lh-condensed mb-3">Launching GitHub Desktop<span class="AnimatedEllipsis"></span></h4>
						<p class="text-gray">If nothing happens, <a href="https://desktop.github.com/">download GitHub Desktop</a> and try again.</p>
						<button type="button" class="btn-link" data-action="click:get-repo#onDetailsToggle">Go back</button>
					</div>
	
					<div class="p-3" data-targets="get-repo.platforms" data-platform="windows" hidden>
						<h4 class="lh-condensed mb-3">Launching GitHub Desktop<span class="AnimatedEllipsis"></span></h4>
						<p class="text-gray">If nothing happens, <a href="https://desktop.github.com/">download GitHub Desktop</a> and try again.</p>
						<button type="button" class="btn-link" data-action="click:get-repo#onDetailsToggle">Go back</button>
					</div>
	
					<div class="p-3" data-targets="get-repo.platforms" data-platform="xcode" hidden>
						<h4 class="lh-condensed mb-3">Launching Xcode<span class="AnimatedEllipsis"></span></h4>
						<p class="text-gray">If nothing happens, <a href="https://developer.apple.com/xcode/">download Xcode</a> and try again.</p>
						<button type="button" class="btn-link" data-action="click:get-repo#onDetailsToggle">Go back</button>
					</div>
	
					<div class="p-3" data-targets="get-repo.platforms" data-platform="visual-studio" hidden>
						<h4 class="lh-condensed mb-3">Launching Visual Studio<span class="AnimatedEllipsis"></span></h4>
						<p class="text-gray">If nothing happens, <a href="https://visualstudio.github.com/">download the GitHub extension for Visual Studio</a> and try again.</p>
						<button type="button" class="btn-link" data-action="click:get-repo#onDetailsToggle">Go back</button>
					</div>
	
				</div>
			</div>
		</details>
	</get-repo>
	
	
				
			</span>
	</div>
	
	
				
	
	<div class="Box mb-3">
		<div class="Box-header Box-header--blue position-relative">
			<h2 class="sr-only">Latest commit</h2>
			<div class="js-details-container Details d-flex rounded-top-1 flex-items-center flex-wrap" data-issue-and-pr-hovercards-enabled>
				
		<div class="flex-shrink-0 ml-n1 mr-n1 mt-n1 mb-n1 hx_avatar_stack_commit">
			
	<div class="AvatarStack flex-self-start ">
		<div class="AvatarStack-body" aria-label="bheisler">
					<a class="avatar avatar-user" data-skip-pjax="true" data-hovercard-type="user" data-hovercard-url="/users/bheisler/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" style="width:24px;height:24px;" href="/bheisler">
						<img height="24" width="24" alt="@bheisler" src="https://avatars3.githubusercontent.com/u/1616938?s=60&amp;v=4" class=" avatar-user" />
	</a>  </div>
	</div>
	
		</div>
		<div class="flex-1 d-flex flex-items-center ml-3 min-width-0">
			<div class="css-truncate css-truncate-overflow text-gray">
				
				<a href="/bheisler/criterion.rs/commits?author=bheisler"
			 class="commit-author user-mention"
			 title="View all commits by bheisler">bheisler</a>
	
	
		
	
					<span class="d-none d-sm-inline">
						<a data-pjax="true" title="Update changelog." class="link-gray-dark" href="/bheisler/criterion.rs/commit/89593658643f9a12949f99f35a6bf906cb4e43c0">Update changelog.</a>
					</span>
			</div>
			<span class="hidden-text-expander ml-2 d-inline-block d-inline-block d-lg-none">
				<button type="button" class="hx_bg-black-fade-15 text-gray-dark ellipsis-expander js-details-target" aria-expanded="false">&hellip;</button>
			</span>
			<div class="d-flex flex-auto flex-justify-end ml-3 flex-items-baseline">
					<include-fragment accept="text/fragment+html" src="/bheisler/criterion.rs/commit/89593658643f9a12949f99f35a6bf906cb4e43c0/rollup?direction=e" class="d-inline"></include-fragment>
				<a href="/bheisler/criterion.rs/commit/89593658643f9a12949f99f35a6bf906cb4e43c0" class="f6 link-gray text-mono ml-2 d-none d-lg-inline" data-pjax>8959365</a>
				<a href="/bheisler/criterion.rs/commit/89593658643f9a12949f99f35a6bf906cb4e43c0" class="link-gray ml-2" data-pjax>
					<relative-time datetime="2020-07-18T16:40:11Z" class="no-wrap">Jul 18, 2020</relative-time>
				</a>
			</div>
		</div>
		<div class="pl-0 pl-md-5 flex-order-1 width-full Details-content--hidden">
				<div class="mt-2">
					<a data-pjax="true" class="link-gray-dark text-bold" href="/bheisler/criterion.rs/commit/89593658643f9a12949f99f35a6bf906cb4e43c0">Update changelog.</a>
				</div>
	
			<div class="d-flex flex-items-center">
				<span class="text-gray-dark text-mono d-lg-none hx_bg-black-fade-15 px-1 rounded-1 text-small mt-2">8959365</span>
			</div>
		</div>
				<div class="flex-shrink-0">
					<h2 class="sr-only">Git stats</h2>
					<ul class="list-style-none d-flex">
						<li class="ml-0 ml-md-3">
							<a data-pjax href="/bheisler/criterion.rs/commits/master" class="pl-3 pr-3 py-3 p-md-0 mt-n3 mb-n3 mr-n3 m-md-0 link-gray-dark no-underline no-wrap">
								<svg text="gray" height="16" class="octicon octicon-history text-gray" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.643 3.143L.427 1.927A.25.25 0 000 2.104V5.75c0 .138.112.25.25.25h3.646a.25.25 0 00.177-.427L2.715 4.215a6.5 6.5 0 11-1.18 4.458.75.75 0 10-1.493.154 8.001 8.001 0 101.6-5.684zM7.75 4a.75.75 0 01.75.75v2.992l2.028.812a.75.75 0 01-.557 1.392l-2.5-1A.75.75 0 017 8.25v-3.5A.75.75 0 017.75 4z"></path></svg>
								<span class="d-none d-sm-inline">
											<strong>1,329</strong>
										<span aria-label="Commits on master" class="text-gray d-none d-lg-inline">commits</span>
								</span>
							</a>
						</li>
					</ul>
				</div>
			</div>
		</div>
		<h2 id="files"  class="sr-only">Files</h2>
		
	
	
			<a class="d-none js-permalink-shortcut" data-hotkey="y" href="/bheisler/criterion.rs/tree/89593658643f9a12949f99f35a6bf906cb4e43c0">Permalink</a>
	
		<div class="include-fragment-error flash flash-error flash-full py-2">
		<svg height="16" class="octicon octicon-alert" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z"></path></svg>
		
			Failed to load latest commit information.
	
	</div>  <div class="js-details-container Details">
			<div role="grid" aria-labelledby="files" class="Details-content--hidden-not-important js-navigation-container js-active-navigation-container d-md-block" data-pjax>
				<div class="sr-only" role="row">
					<div role="columnheader">Type</div>
					<div role="columnheader">Name</div>
					<div role="columnheader" class="d-none d-md-block">Latest commit message</div>
					<div role="columnheader">Commit time</div>
				</div>
	
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="blue-3" aria-label="Directory" height="16" class="octicon octicon-file-directory color-blue-3" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="bencher_compat" id="f1520d3ae82f166d69d223dd7a41078e-5a020ecf6491a594c8545c221223bce3f3665b8d" href="/bheisler/criterion.rs/tree/master/bencher_compat">bencher_compat</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Bump version numbers." class="link-gray" href="/bheisler/criterion.rs/commit/3fcbcd237e14306d102f2ea2f1285cd6285e086c">Bump version numbers.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-06-29T17:46:12Z" class="no-wrap">Jun 30, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="blue-3" aria-label="Directory" height="16" class="octicon octicon-file-directory color-blue-3" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="benches" id="808a2c94ce7409280a94738efeb6f3d1-8c9b6095a2312e7e0545739dc7fc647441f2947e" href="/bheisler/criterion.rs/tree/master/benches">benches</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Escape single-quotes in benchmark names when generating Gnuplot scripts. Fixes #422." class="link-gray" href="/bheisler/criterion.rs/commit/9714ddb7881cf1295578395327ece3ccad74401a">Escape single-quotes in benchmark names when generating Gnuplot scrip…</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-06-28T21:53:23Z" class="no-wrap">Jun 29, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="blue-3" aria-label="Directory" height="16" class="octicon octicon-file-directory color-blue-3" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="book" id="821f03288846297c2cf43c34766a38f7-9bbd3b3210c934b022310e0a7865b489abfdbb22" href="/bheisler/criterion.rs/tree/master/book">book</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Update docs for latest release of cargo-criterion." class="link-gray" href="/bheisler/criterion.rs/commit/3764dbc4eb51edee9f73efef701140758f9e6894">Update docs for latest release of cargo-criterion.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-07-06T23:30:10Z" class="no-wrap">Jul 7, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="blue-3" aria-label="Directory" height="16" class="octicon octicon-file-directory color-blue-3" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="ci" id="35ea51baf1fe7f0142ad5f950855dde0-13e26f45da3d0da5e6ab99d234fe9763dcda2afc" href="/bheisler/criterion.rs/tree/master/ci">ci</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Disable incremental compilation in CI." class="link-gray" href="/bheisler/criterion.rs/commit/3168189cbe67d4a1367809f20f42a833eb46cc84">Disable incremental compilation in CI.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-07-04T00:44:52Z" class="no-wrap">Jul 4, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="blue-3" aria-label="Directory" height="16" class="octicon octicon-file-directory color-blue-3" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="macro" id="eb320f0c2b6a25b48ca861a120eea902-dade7a522ae1601a1883e98bed86b804a55e3d02" href="/bheisler/criterion.rs/tree/master/macro">macro</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Bump version numbers." class="link-gray" href="/bheisler/criterion.rs/commit/3fcbcd237e14306d102f2ea2f1285cd6285e086c">Bump version numbers.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-06-29T17:46:12Z" class="no-wrap">Jun 30, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="blue-3" aria-label="Directory" height="16" class="octicon octicon-file-directory color-blue-3" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="plot" id="32fa6e1b78a9d4028953e60564a2aa4c-99533c845f78ada7d57b88231448f0910933c191" href="/bheisler/criterion.rs/tree/master/plot">plot</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Bump version numbers." class="link-gray" href="/bheisler/criterion.rs/commit/3fcbcd237e14306d102f2ea2f1285cd6285e086c">Bump version numbers.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-06-29T17:46:12Z" class="no-wrap">Jun 30, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="blue-3" aria-label="Directory" height="16" class="octicon octicon-file-directory color-blue-3" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="src" id="25d902c24283ab8cfbac54dfa101ad31-a7840ca1714ad9d29df5ee5c9cfb6c167d9c4e68" href="/bheisler/criterion.rs/tree/master/src">src</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Minor change to ensure we always do at least one iteration per sample." class="link-gray" href="/bheisler/criterion.rs/commit/77599c2cccb2232788714af08db03594dab441b2">Minor change to ensure we always do at least one iteration per sample.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-07-18T16:38:45Z" class="no-wrap">Jul 19, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="blue-3" aria-label="Directory" height="16" class="octicon octicon-file-directory color-blue-3" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="tests" id="b61a6d542f9036550ba9c401c80f00ef-eebf09b6a7b5a34ed7d850b3e4a9fb9c223a2064" href="/bheisler/criterion.rs/tree/master/tests">tests</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Give nice error message on broken benchmarks that take zero time." class="link-gray" href="/bheisler/criterion.rs/commit/44c07f131ab6e4d0822533b1a0693487f99aae6c">Give nice error message on broken benchmarks that take zero time.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-07-18T16:38:13Z" class="no-wrap">Jul 19, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title=".editorconfig" id="1e70daafb475c0ce3fef7d2728279182-8f7f6659910528668ed9f344c533bf36730b78c6" href="/bheisler/criterion.rs/blob/master/.editorconfig">.editorconfig</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Add editorconfig file and standardize on unix-style line endings." class="link-gray" href="/bheisler/criterion.rs/commit/e0b6e21a7bc1b93abd6fe9a1d3bbb05c9b10ccd3">Add editorconfig file and standardize on unix-style line endings.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2018-01-14T01:24:09Z" class="no-wrap">Jan 14, 2018</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title=".gitignore" id="a084b794bc0759e7a6b77810e01874f2-a8a47d521c0abc11c7ba933c0f9beed2dbe110f0" href="/bheisler/criterion.rs/blob/master/.gitignore">.gitignore</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Plotters Integration
	
	This change integrates the pure Rust plotting crate, Plotters, to
	Criterion. This will allows the user doesn&#39;t relies on Gnuplot to
	generate the benchmark data visualization.
	
	After the change, Criterion won&#39;t have any change unless:
	1. Gnuplot is not installed: Preivously we completely disable HTML
	report, now we using Plotters to generate the report visualization
	instead.
	2. If `--prefer-plotters` is passed to benchmark, Criterion will prefer
	Plotters based plotting backend.
	
	Additional Notes:
	
	1. This change refactors the plotting code to use dynamic dispatch
	2. The Plotters based plotting backend is implemented
	3. Commandline options is added" class="link-gray" href="/bheisler/criterion.rs/commit/adac441859cf279602691761120d264cb8eaf660">Plotters Integration</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2019-11-27T22:17:42Z" class="no-wrap">Nov 28, 2019</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title=".travis.yml" id="354f30a63fb0907d4ad57269548329e3-854a53503717ca5787f3a8d08290ac73d2f03848" href="/bheisler/criterion.rs/blob/master/.travis.yml">.travis.yml</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Remove the Cargo registry cache, since it changes on most builds" class="link-gray" href="/bheisler/criterion.rs/commit/cebda3d4a30fab4605da4947f47eb64ed5d269db">Remove the Cargo registry cache, since it changes on most builds</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-07-04T01:38:05Z" class="no-wrap">Jul 4, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="CHANGELOG.md" id="4ac32a78649ca5bdd8e0ba38b7006a1e-2b6cfa638c73702b00aa1eec3fb48f4a4a73934d" href="/bheisler/criterion.rs/blob/master/CHANGELOG.md">CHANGELOG.md</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Update changelog." class="link-gray" href="/bheisler/criterion.rs/commit/89593658643f9a12949f99f35a6bf906cb4e43c0">Update changelog.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-07-18T16:40:11Z" class="no-wrap">Jul 19, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="CONTRIBUTING.md" id="6a3371457528722a734f3c51d9238c13-3442998938fc9bc583cb1e5c9fd93c4e88c51c86" href="/bheisler/criterion.rs/blob/master/CONTRIBUTING.md">CONTRIBUTING.md</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Update all links to point to the new repository URL." class="link-gray" href="/bheisler/criterion.rs/commit/cc7218f6e1e8a061b5aa79d1666984c8b71e506e">Update all links to point to the new repository URL.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2018-12-08T22:49:17Z" class="no-wrap">Dec 9, 2018</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="Cargo.toml" id="80398c5faae3c069e4e6aa2ed11b28c0-1dc9def8e84c35ddb320a57630bdd09208d7012e" href="/bheisler/criterion.rs/blob/master/Cargo.toml">Cargo.toml</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Bump version numbers." class="link-gray" href="/bheisler/criterion.rs/commit/3fcbcd237e14306d102f2ea2f1285cd6285e086c">Bump version numbers.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-06-29T17:46:12Z" class="no-wrap">Jun 30, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="LICENSE-APACHE" id="6876cecbe03bb3ada105fce902f4fba2-16fe87b06e802f094b3fbb0894b137bca2b16ef1" itemprop="license" href="/bheisler/criterion.rs/blob/master/LICENSE-APACHE">LICENSE-APACHE</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Initial commit" class="link-gray" href="/bheisler/criterion.rs/commit/34d8ef4be7a4d8ad1de89835f82de94f05536b36">Initial commit</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2014-09-22T22:05:05Z" class="no-wrap">Sep 23, 2014</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="LICENSE-MIT" id="7f0bdbc0a0545831259b66259ac6b604-ac0fda00385af1cd74acd663ea8fc73c56a94fd3" itemprop="license" href="/bheisler/criterion.rs/blob/master/LICENSE-MIT">LICENSE-MIT</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Initial commit" class="link-gray" href="/bheisler/criterion.rs/commit/34d8ef4be7a4d8ad1de89835f82de94f05536b36">Initial commit</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2014-09-22T22:05:05Z" class="no-wrap">Sep 23, 2014</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="README.md" id="04c6e90faac2675aa89e2176d2eec7d8-e33e84ad14b328b01a27db4b405165f35dff4ee9" href="/bheisler/criterion.rs/blob/master/README.md">README.md</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Bump oldest-working Rust version to 1.36." class="link-gray" href="/bheisler/criterion.rs/commit/520ebdf498cb5d646f8a4a902cb3aa7059b51e25">Bump oldest-working Rust version to 1.36.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2020-06-29T16:39:51Z" class="no-wrap">Jun 30, 2020</time-ago>
						</div>
	
					</div>
					<div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
						<div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
								<svg color="gray-light" aria-label="File" height="16" class="octicon octicon-file text-gray-light" viewBox="0 0 16 16" version="1.1" width="16" role="img"><path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path></svg>
	
						</div>
	
						<div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
							<span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open link-gray-dark" title="appveyor.yml" id="180360612c6b8c4ed830919bbb4dd459-c145df3b27db4b46832121a3a9c88cbe90657306" href="/bheisler/criterion.rs/blob/master/appveyor.yml">appveyor.yml</a></span>
						</div>
	
						<div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3 commit-message">
								<span class="css-truncate css-truncate-target d-block width-fit">
											<a data-pjax="true" title="Fix Appveyor builds." class="link-gray" href="/bheisler/criterion.rs/commit/1f8434acc3199fb1337f16550ec2f57d2199b18e">Fix Appveyor builds.</a>
								</span>
						</div>
	
						<div role="gridcell" class="text-gray-light text-right" style="width:100px;">
								<time-ago datetime="2018-01-21T16:03:16Z" class="no-wrap">Jan 22, 2018</time-ago>
						</div>
	
					</div>
			</div>
			<div class="Details-content--shown Box-footer d-md-none p-0">
				<button type="button" class="d-block btn-link js-details-target width-full px-3 py-2" aria-expanded="false">
					View code
				</button>
			</div>
		</div>
	
	
	
	
	</div>
	
		<div id="readme" class="Box md js-code-block-container Box--responsive">
			<div class="Box-header d-flex flex-items-center flex-justify-between bg-white border-bottom-0">
				<h2 class="Box-title pr-3">
					README.md
				</h2>
			</div>
					<div class="Popover anim-scale-in js-tagsearch-popover"
			 hidden
			 data-tagsearch-url="/bheisler/criterion.rs/find-definition"
			 data-tagsearch-ref="master"
			 data-tagsearch-path="README.md"
			 data-tagsearch-lang="Markdown"
			 data-hydro-click="{&quot;event_type&quot;:&quot;code_navigation.click_on_symbol&quot;,&quot;payload&quot;:{&quot;action&quot;:&quot;click_on_symbol&quot;,&quot;repository_id&quot;:20188259,&quot;ref&quot;:&quot;master&quot;,&quot;language&quot;:&quot;Markdown&quot;,&quot;originating_url&quot;:&quot;https://github.com/bheisler/criterion.rs&quot;,&quot;user_id&quot;:7714080}}"
			 data-hydro-click-hmac="9a2476b7bc12752fbfa6f1a9ba190ac1f9333789f16c1e3574bd67948d01edcb">
		<div class="Popover-message Popover-message--large Popover-message--top-left TagsearchPopover mt-1 mb-4 mx-auto Box box-shadow-large">
			<div class="TagsearchPopover-content js-tagsearch-popover-content overflow-auto" style="will-change:transform;">
			</div>
		</div>
	</div>
	
				<div class="Box-body px-5 pb-5">
					<article class="markdown-body entry-content container-lg" itemprop="text"><h1 align="center"><a id="user-content-criterionrs" class="anchor" aria-hidden="true" href="#criterionrs"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Criterion.<span></span>rs</h1>
	<div align="center">Statistics-driven Microbenchmarking in Rust</div>
	<div align="center">
		<a href="https://bheisler.github.io/criterion.rs/book/getting_started.html" rel="nofollow">Getting Started</a>
			|
			<a href="https://bheisler.github.io/criterion.rs/book/index.html" rel="nofollow">User Guide</a>
			|
			<a href="https://bheisler.github.io/criterion.rs/criterion/" rel="nofollow">Master API Docs</a>
			|
			<a href="https://docs.rs/crate/criterion/" rel="nofollow">Released API Docs</a>
			|
			<a href="https://github.com/bheisler/criterion.rs/blob/master/CHANGELOG.md">Changelog</a>
	</div>
	<div align="center">
		<a href="https://travis-ci.org/bheisler/criterion.rs" rel="nofollow">
					<img src="https://camo.githubusercontent.com/3eb1b9dcab72731722086a9c138261ded747cfad/68747470733a2f2f7472617669732d63692e6f72672f62686569736c65722f637269746572696f6e2e72732e7376673f6272616e63683d6d6173746572" alt="Travis-CI" data-canonical-src="https://travis-ci.org/bheisler/criterion.rs.svg?branch=master" style="max-width:100%;">
			</a>
			|
			<a href="https://ci.appveyor.com/project/bheisler/criterion-rs-vt9fl" rel="nofollow">
					<img src="https://camo.githubusercontent.com/4a242f4a014551b8fba6f0f09fa3fc98a3d6247d/68747470733a2f2f63692e6170707665796f722e636f6d2f6170692f70726f6a656374732f7374617475732f34323535616473396374707570636c323f7376673d74727565" alt="Appveyor" data-canonical-src="https://ci.appveyor.com/api/projects/status/4255ads9ctpupcl2?svg=true" style="max-width:100%;">
			</a>
			|
			<a href="https://crates.io/crates/criterion" rel="nofollow">
					<img src="https://camo.githubusercontent.com/698fe3854bbf39f9e70e7f036399ac7801d625d9/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f637269746572696f6e2e737667" alt="Crates.io" data-canonical-src="https://img.shields.io/crates/v/criterion.svg" style="max-width:100%;">
			</a>
	</div>
	<p>Criterion.<span></span>rs helps you write fast code by detecting and measuring performance improvements or regressions, even small ones, quickly and accurately. You can optimize with confidence, knowing how each change affects the performance of your code.</p>
	<h2><a id="user-content-table-of-contents" class="anchor" aria-hidden="true" href="#table-of-contents"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Table of Contents</h2>
	<ul>
	<li><a href="#table-of-contents">Table of Contents</a>
	<ul>
	<li><a href="#features">Features</a></li>
	<li><a href="#quickstart">Quickstart</a></li>
	<li><a href="#goals">Goals</a></li>
	<li><a href="#contributing">Contributing</a></li>
	<li><a href="#compatibility-policy">Compatibility Policy</a></li>
	<li><a href="#maintenance">Maintenance</a></li>
	<li><a href="#license">License</a></li>
	<li><a href="#related-projects">Related Projects</a></li>
	<li><a href="#criterionrs-extensions">Criterion.rs Extensions</a></li>
	</ul>
	</li>
	</ul>
	<h3><a id="user-content-features" class="anchor" aria-hidden="true" href="#features"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Features</h3>
	<ul>
	<li><strong>Statistics</strong>: Statistical analysis detects if, and by how much, performance has changed since the last benchmark run</li>
	<li><strong>Charts</strong>: Uses <a href="http://www.gnuplot.info/" rel="nofollow">gnuplot</a> to generate detailed graphs of benchmark results</li>
	<li><strong>Stable-compatible</strong>: Benchmark your code without installing nightly Rust</li>
	</ul>
	<h3><a id="user-content-quickstart" class="anchor" aria-hidden="true" href="#quickstart"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Quickstart</h3>
	<p>In order to generate plots, you must have <a href="http://www.gnuplot.info/" rel="nofollow">gnuplot</a> installed. See the gnuplot website for installation instructions. See <a href="#compatibility-policy">Compatibility Policy</a> for details on the minimum supported Rust version.</p>
	<p>To start with Criterion.<span></span>rs, add the following to your <code>Cargo.toml</code> file:</p>
	<div class="highlight highlight-source-toml"><pre>[<span class="pl-en">dev-dependencies</span>]
	<span class="pl-smi">criterion</span> = <span class="pl-s"><span class="pl-pds">"</span>0.3<span class="pl-pds">"</span></span>
	
	[[<span class="pl-en">bench</span>]]
	<span class="pl-smi">name</span> = <span class="pl-s"><span class="pl-pds">"</span>my_benchmark<span class="pl-pds">"</span></span>
	<span class="pl-smi">harness</span> = <span class="pl-c1">false</span></pre></div>
	<p>Next, define a benchmark by creating a file at <code>$PROJECT/benches/my_benchmark.rs</code> with the following contents:</p>
	<div class="highlight highlight-source-rust"><pre><span class="pl-k">use</span> criterion<span class="pl-k">::</span>{black_box, criterion_group, criterion_main, Criterion};
	
	<span class="pl-k">fn</span> <span class="pl-en">fibonacci</span>(n: <span class="pl-k">u64</span>) -&gt; <span class="pl-k">u64</span> {
			<span class="pl-k">match</span> n {
					<span class="pl-c1">0</span> <span class="pl-k">=&gt;</span> <span class="pl-c1">1</span>,
					<span class="pl-c1">1</span> <span class="pl-k">=&gt;</span> <span class="pl-c1">1</span>,
					n <span class="pl-k">=&gt;</span> <span class="pl-en">fibonacci</span>(n<span class="pl-k">-</span><span class="pl-c1">1</span>) <span class="pl-k">+</span> <span class="pl-en">fibonacci</span>(n<span class="pl-k">-</span><span class="pl-c1">2</span>),
			}
	}
	
	<span class="pl-k">fn</span> <span class="pl-en">criterion_benchmark</span>(c: <span class="pl-k">&amp;</span><span class="pl-k">mut</span> Criterion) {
			c.<span class="pl-en">bench_function</span>(<span class="pl-s">"fib 20"</span>, <span class="pl-k">|</span>b<span class="pl-k">|</span> b.<span class="pl-en">iter</span>(<span class="pl-k">||</span> <span class="pl-en">fibonacci</span>(<span class="pl-en">black_box</span>(<span class="pl-c1">20</span>))));
	}
	
	<span class="pl-en">criterion_group!</span>(benches, criterion_benchmark);
	<span class="pl-en">criterion_main!</span>(benches);</pre></div>
	<p>Finally, run this benchmark with <code>cargo bench</code>. You should see output similar to the following:</p>
	<pre><code>     Running target/release/deps/example-423eedc43b2b3a93
	fib 20                  time:   [26.029 us 26.251 us 26.505 us]
	Found 11 outliers among 99 measurements (11.11%)
		6 (6.06%) high mild
		5 (5.05%) high severe
	</code></pre>
	<p>See the <a href="https://bheisler.github.io/criterion.rs/book/getting_started.html" rel="nofollow">Getting Started</a> guide for more details.</p>
	<h3><a id="user-content-goals" class="anchor" aria-hidden="true" href="#goals"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Goals</h3>
	<p>The primary goal of Criterion.<span></span>rs is to provide a powerful and statistically rigorous tool for measuring the performance of code, preventing performance regressions and accurately measuring optimizations. Additionally, it should be as programmer-friendly as possible and make it easy to create reliable, useful benchmarks, even for programmers without an advanced background in statistics.</p>
	<h3><a id="user-content-contributing" class="anchor" aria-hidden="true" href="#contributing"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Contributing</h3>
	<p>First, thank you for contributing.</p>
	<p>One great way to contribute to Criterion.<span></span>rs is to use it for your own benchmarking needs and report your experiences, file and comment on issues, etc.</p>
	<p>Code or documentation improvements in the form of pull requests are also welcome. If you're not
	sure what to work on, try checking the
	<a href="https://github.com/bheisler/criterion.rs/issues?q=is%3Aissue+is%3Aopen+label%3ABeginner">Beginner label</a>.</p>
	<p>If your issues or pull requests have no response after a few days, feel free to ping me (@bheisler).</p>
	<p>For more details, see the <a href="https://github.com/bheisler/criterion.rs/blob/master/CONTRIBUTING.md">CONTRIBUTING.md file</a>.</p>
	<h3><a id="user-content-compatibility-policy" class="anchor" aria-hidden="true" href="#compatibility-policy"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Compatibility Policy</h3>
	<p>Criterion.<span></span>rs supports the last three stable minor releases of Rust. At time of
	writing, this means Rust 1.40 or later. Older versions may work, but are not tested or guaranteed.</p>
	<p>Currently, the oldest version of Rust believed to work is 1.36. Future versions of Criterion.<span></span>rs may
	break support for such old versions, and this will not be considered a breaking change. If you
	require Criterion.<span></span>rs to work on old versions of Rust, you will need to stick to a
	specific patch version of Criterion.<span></span>rs.</p>
	<h3><a id="user-content-maintenance" class="anchor" aria-hidden="true" href="#maintenance"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Maintenance</h3>
	<p>Criterion.<span></span>rs was originally created by Jorge Aparicio (@japaric) and is currently being maintained by Brook Heisler (@bheisler).</p>
	<h3><a id="user-content-license" class="anchor" aria-hidden="true" href="#license"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>License</h3>
	<p>Criterion.<span></span>rs is dual licensed under the Apache 2.0 license and the MIT license.</p>
	<h3><a id="user-content-related-projects" class="anchor" aria-hidden="true" href="#related-projects"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Related Projects</h3>
	<ul>
	<li><a href="https://github.com/bluss/bencher">bencher</a> - A port of the libtest benchmark runner to stable Rust</li>
	<li><a href="http://www.serpentine.com/criterion/" rel="nofollow">criterion</a> - The Haskell microbenchmarking library that inspired Criterion.<span></span>rs</li>
	<li><a href="https://github.com/BurntSushi/cargo-benchcmp">cargo-benchcmp</a> - Cargo subcommand to compare the output of two libtest or bencher benchmark runs</li>
	<li><a href="https://github.com/ferrous-systems/flamegraph">cargo-flamegraph</a> - Cargo subcommand to profile an executable and produce a flamegraph</li>
	</ul>
	<h3><a id="user-content-criterionrs-extensions" class="anchor" aria-hidden="true" href="#criterionrs-extensions"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Criterion.rs Extensions</h3>
	<ul>
	<li><a href="https://crates.io/crates/criterion-cycles-per-byte" rel="nofollow">criterion-cycles-per-byte</a> - A custom-measurement plugin that counts the number of CPU cycles used by the benchmark</li>
	<li><a href="https://crates.io/crates/criterion-perf-events" rel="nofollow">criterion-perf-events</a> - A custom-measurement plugin that counts perf events created by the benchmark</li>
	</ul>
	</article>
				</div>
		</div>
	
	
	</div>
			<div class="flex-shrink-0 col-12 col-md-3">
							
	
				<div class="BorderGrid BorderGrid--spacious" data-pjax>
					<div class="BorderGrid-row hide-sm hide-md">
						<div class="BorderGrid-cell">
							<h2 class="mb-3 h4">About</h2>
	
			<p class="f4 mt-3">
				Statistics-driven benchmarking library for Rust
			</p>
	
		<h3 class="sr-only">Topics</h3>
		<div class="mt-3">
				<div class="f6">
				<a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:rust" href="/topics/rust" title="Topic: rust" class="topic-tag topic-tag-link ">
		rust
	</a>
				<a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:benchmark" href="/topics/benchmark" title="Topic: benchmark" class="topic-tag topic-tag-link ">
		benchmark
	</a>
				<a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:statistics" href="/topics/statistics" title="Topic: statistics" class="topic-tag topic-tag-link ">
		statistics
	</a>
				<a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:gnuplot" href="/topics/gnuplot" title="Topic: gnuplot" class="topic-tag topic-tag-link ">
		gnuplot
	</a>
				<a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:criterion" href="/topics/criterion" title="Topic: criterion" class="topic-tag topic-tag-link ">
		criterion
	</a>
		</div>
	
		</div>
	
		<h3 class="sr-only">Resources</h3>
		<div class="mt-3">
			<a class="muted-link" href="#readme">
				<svg mr="2" height="16" class="octicon octicon-book mr-2" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M0 1.75A.75.75 0 01.75 1h4.253c1.227 0 2.317.59 3 1.501A3.744 3.744 0 0111.006 1h4.245a.75.75 0 01.75.75v10.5a.75.75 0 01-.75.75h-4.507a2.25 2.25 0 00-1.591.659l-.622.621a.75.75 0 01-1.06 0l-.622-.621A2.25 2.25 0 005.258 13H.75a.75.75 0 01-.75-.75V1.75zm8.755 3a2.25 2.25 0 012.25-2.25H14.5v9h-3.757c-.71 0-1.4.201-1.992.572l.004-7.322zm-1.504 7.324l.004-5.073-.002-2.253A2.25 2.25 0 005.003 2.5H1.5v9h3.757a3.75 3.75 0 011.994.574z"></path></svg>
				Readme
	</a>  </div>
	
		<h3 class="sr-only">License</h3>
		<div class="mt-3">
			<a href="/bheisler/criterion.rs/blob/master/LICENSE-APACHE" class="muted-link" >
				<svg mr="2" height="16" class="octicon octicon-law mr-2" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M8.75.75a.75.75 0 00-1.5 0V2h-.984c-.305 0-.604.08-.869.23l-1.288.737A.25.25 0 013.984 3H1.75a.75.75 0 000 1.5h.428L.066 9.192a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.514 3.514 0 00.686.45A4.492 4.492 0 003 11c.88 0 1.556-.22 2.023-.454a3.515 3.515 0 00.686-.45l.045-.04.016-.015.006-.006.002-.002.001-.002L5.25 9.5l.53.53a.75.75 0 00.154-.838L3.822 4.5h.162c.305 0 .604-.08.869-.23l1.289-.737a.25.25 0 01.124-.033h.984V13h-2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-2.5V3.5h.984a.25.25 0 01.124.033l1.29.736c.264.152.563.231.868.231h.162l-2.112 4.692a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.517 3.517 0 00.686.45A4.492 4.492 0 0013 11c.88 0 1.556-.22 2.023-.454a3.512 3.512 0 00.686-.45l.045-.04.01-.01.006-.005.006-.006.002-.002.001-.002-.529-.531.53.53a.75.75 0 00.154-.838L13.823 4.5h.427a.75.75 0 000-1.5h-2.234a.25.25 0 01-.124-.033l-1.29-.736A1.75 1.75 0 009.735 2H8.75V.75zM1.695 9.227c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L3 6.327l-1.305 2.9zm10 0c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L13 6.327l-1.305 2.9z"></path></svg>
					View license
			</a>
		</div>
	
						</div>
					</div>
						<div class="BorderGrid-row">
							<div class="BorderGrid-cell">
								<h2 class="h4 mb-3">
		<a href="/bheisler/criterion.rs/releases" class="link-gray-dark no-underline ">
			Releases
	</a></h2>
	
			<a class="link-gray-dark no-underline" href="/bheisler/criterion.rs/releases">
				<svg height="16" class="octicon octicon-tag" viewBox="0 0 16 16" version="1.1" width="16" aria-hidden="true"><path fill-rule="evenodd" d="M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z"></path></svg>
				<span class="text-bold">19</span>
				<span class="text-gray">tags</span>
	</a>
							</div>
						</div>
						<div class="BorderGrid-row">
							<div class="BorderGrid-cell">
								<h2 class="h4 mb-3">
		<a href="/bheisler/criterion.rs/packages" class="link-gray-dark no-underline ">
			Packages <span title="0" hidden="hidden" class="Counter ">0</span>
	</a></h2>
	
	
				<div class="text-small">
					No packages published <br>
				</div>
	
	
	
							</div>
						</div>
						<div class="BorderGrid-row" hidden>
							<div class="BorderGrid-cell">
								<include-fragment src="/bheisler/criterion.rs/used_by_list" accept="text/fragment+html">
	</include-fragment>
							</div>
						</div>
						<div class="BorderGrid-row">
							<div class="BorderGrid-cell">
								<h2 class="h4 mb-3">
		<a href="/bheisler/criterion.rs/graphs/contributors" class="link-gray-dark no-underline ">
			Contributors <span title="65" class="Counter ">65</span>
	</a></h2>
	
	
			
		<ul class="list-style-none d-flex flex-wrap mb-n2">
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/bheisler/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/bheisler">
						<img class="d-block avatar-user" src="https://avatars3.githubusercontent.com/u/1616938?s=64&amp;v=4" width="32" height="32" alt="@bheisler" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/japaric/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/japaric">
						<img class="d-block avatar-user" src="https://avatars2.githubusercontent.com/u/5018213?s=64&amp;v=4" width="32" height="32" alt="@japaric" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/faern/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/faern">
						<img class="d-block avatar-user" src="https://avatars1.githubusercontent.com/u/332294?s=64&amp;v=4" width="32" height="32" alt="@faern" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/RReverser/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/RReverser">
						<img class="d-block avatar-user" src="https://avatars3.githubusercontent.com/u/557590?s=64&amp;v=4" width="32" height="32" alt="@RReverser" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/gz/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/gz">
						<img class="d-block avatar-user" src="https://avatars1.githubusercontent.com/u/127654?s=64&amp;v=4" width="32" height="32" alt="@gz" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/bsteinb/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/bsteinb">
						<img class="d-block avatar-user" src="https://avatars0.githubusercontent.com/u/3883096?s=64&amp;v=4" width="32" height="32" alt="@bsteinb" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/Eh2406/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/Eh2406">
						<img class="d-block avatar-user" src="https://avatars1.githubusercontent.com/u/3709504?s=64&amp;v=4" width="32" height="32" alt="@Eh2406" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/gnzlbg/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/gnzlbg">
						<img class="d-block avatar-user" src="https://avatars3.githubusercontent.com/u/904614?s=64&amp;v=4" width="32" height="32" alt="@gnzlbg" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/damienstanton/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/damienstanton">
						<img class="d-block avatar-user" src="https://avatars2.githubusercontent.com/u/4822375?s=64&amp;v=4" width="32" height="32" alt="@damienstanton" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/laumann/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/laumann">
						<img class="d-block avatar-user" src="https://avatars3.githubusercontent.com/u/390428?s=64&amp;v=4" width="32" height="32" alt="@laumann" />
	</a>      </li>
				<li class="mb-2 mr-2">
					<a class="" data-hovercard-type="user" data-hovercard-url="/users/pierrechevalier83/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/pierrechevalier83">
						<img class="d-block avatar-user" src="https://avatars3.githubusercontent.com/u/5790907?s=64&amp;v=4" width="32" height="32" alt="@pierrechevalier83" />
	</a>      </li>
		</ul>
	
	
		<div class="mt-3">
			<a href="/bheisler/criterion.rs/graphs/contributors" class="text-small">
				+ 54 contributors
	</a></div>
							</div>
						</div>
						<div class="BorderGrid-row">
							<div class="BorderGrid-cell">
								<h2 class="h4 mb-3">Languages</h2>
	<div class="mb-2">
		<span class="Progress ">
			<span itemprop="keywords" aria-label="Rust 84.3" style="background-color: #dea584;width: 84.3%;" class="Progress-item "></span>
			<span itemprop="keywords" aria-label="HTML 15.4" style="background-color: #e34c26;width: 15.4%;" class="Progress-item "></span>
			<span itemprop="keywords" aria-label="Other 0.3" style="background-color: #ededed;width: 0.3%;" class="Progress-item "></span>
	</span></div>
	<ul class="list-style-none">
			<li class="d-inline">
				<a class="d-inline-flex flex-items-center flex-nowrap link-gray no-underline text-small mr-3" href="/bheisler/criterion.rs/search?l=rust"  data-ga-click="Repository, language stats search click, location:repo overview">
					<svg class="octicon octicon-dot-fill mr-2" style="color:#dea584;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
					<span class="text-gray-dark text-bold mr-1">Rust</span>
					<span>84.3%</span>
				</a>
			</li>
			<li class="d-inline">
				<a class="d-inline-flex flex-items-center flex-nowrap link-gray no-underline text-small mr-3" href="/bheisler/criterion.rs/search?l=html"  data-ga-click="Repository, language stats search click, location:repo overview">
					<svg class="octicon octicon-dot-fill mr-2" style="color:#e34c26;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
					<span class="text-gray-dark text-bold mr-1">HTML</span>
					<span>15.4%</span>
				</a>
			</li>
			<li class="d-inline">
				<span class="d-inline-flex flex-items-center flex-nowrap text-small mr-3">
					<svg class="octicon octicon-dot-fill mr-2" style="color:#ededed;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
					<span class="text-gray-dark text-bold mr-1">Other</span>
					<span>0.3%</span>
				</span>
			</li>
	</ul>
	
							</div>
						</div>
				</div>
	
	</div></div>
	
		</div>
	</div>
	
			</main>
		</div>
	
		</div>
	
					
	<div class="footer container-xl width-full p-responsive" role="contentinfo">
		<div class="position-relative d-flex flex-row-reverse flex-lg-row flex-wrap flex-lg-nowrap flex-justify-center flex-lg-justify-between pt-6 pb-2 mt-6 f6 text-gray border-top border-gray-light ">
			<ul class="list-style-none d-flex flex-wrap col-12 col-lg-5 flex-justify-center flex-lg-justify-between mb-2 mb-lg-0">
				<li class="mr-3 mr-lg-0">&copy; 2020 GitHub, Inc.</li>
					<li class="mr-3 mr-lg-0"><a data-ga-click="Footer, go to terms, text:terms" href="https://github.com/site/terms">Terms</a></li>
					<li class="mr-3 mr-lg-0"><a data-ga-click="Footer, go to privacy, text:privacy" href="https://github.com/site/privacy">Privacy</a></li>
					<li class="mr-3 mr-lg-0"><a data-ga-click="Footer, go to security, text:security" href="https://github.com/security">Security</a></li>
					<li class="mr-3 mr-lg-0"><a href="https://githubstatus.com/" data-ga-click="Footer, go to status, text:status">Status</a></li>
					<li><a data-ga-click="Footer, go to help, text:help" href="https://docs.github.com">Help</a></li>
	
			</ul>
	
			<a aria-label="Homepage" title="GitHub" class="footer-octicon d-none d-lg-block mx-lg-4" href="https://github.com">
				<svg height="24" class="octicon octicon-mark-github" viewBox="0 0 16 16" version="1.1" width="24" aria-hidden="true"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
	</a>
		 <ul class="list-style-none d-flex flex-wrap col-12 col-lg-5 flex-justify-center flex-lg-justify-between mb-2 mb-lg-0">
					<li class="mr-3 mr-lg-0"><a data-ga-click="Footer, go to contact, text:contact" href="https://github.com/contact">Contact GitHub</a></li>
					<li class="mr-3 mr-lg-0"><a href="https://github.com/pricing" data-ga-click="Footer, go to Pricing, text:Pricing">Pricing</a></li>
				<li class="mr-3 mr-lg-0"><a href="https://docs.github.com" data-ga-click="Footer, go to api, text:api">API</a></li>
				<li class="mr-3 mr-lg-0"><a href="https://services.github.com" data-ga-click="Footer, go to training, text:training">Training</a></li>
					<li class="mr-3 mr-lg-0"><a href="https://github.blog" data-ga-click="Footer, go to blog, text:blog">Blog</a></li>
					<li><a data-ga-click="Footer, go to about, text:about" href="https://github.com/about">About</a></li>
			</ul>
		</div>
		<div class="d-flex flex-justify-center pb-6">
			<span class="f6 text-gray-light"></span>
		</div>
	</div>
	
	
	
		<div id="ajax-error-message" class="ajax-error-message flash flash-error">
			<svg class="octicon octicon-alert" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z"></path></svg>
			<button type="button" class="flash-close js-ajax-error-dismiss" aria-label="Dismiss error">
				<svg class="octicon octicon-x" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path></svg>
			</button>
			You can’t perform that action at this time.
		</div>
	
	
			<script crossorigin="anonymous" async="async" integrity="sha512-bn/3rKJzBl2H64K38R8KaVcT26vKK7BJQC59lwYc+9fjlHzmy0fwh+hzBtsgTdhIi13dxjzNKWhdSN8WTM9qUw==" type="application/javascript" id="js-conditional-compat" data-src="https://github.githubassets.com/assets/compat-bootstrap-6e7ff7ac.js"></script>
			<script crossorigin="anonymous" integrity="sha512-CxjaMepCmi+z0LTeztU2S8qGD25LyHD6j9t0RSPevy63trFWJVwUM6ipAVLgtpMBBgZ53wq8JPkSeQ6ruaZL2w==" type="application/javascript" src="https://github.githubassets.com/assets/environment-bootstrap-0b18da31.js"></script>
			<script crossorigin="anonymous" async="async" integrity="sha512-5UbilDymVKGbWyRjGkrLl3tWYgKJfHf3h4WHvztG22A1Ng8UMq5Dd7TR3rMYp0u23ldjDP7cITERzNtFwP5bGA==" type="application/javascript" src="https://github.githubassets.com/assets/vendor-e546e294.js"></script>
			<script crossorigin="anonymous" async="async" integrity="sha512-8511eXxi5iXu22XSVAQ9uhdrqjtWGE2kDEE44cR+UbszX/pZgNpwGXRpadal+hIKSv+BlkPS/jyojgOJyvxNLw==" type="application/javascript" src="https://github.githubassets.com/assets/frameworks-f39d7579.js"></script>
			
			<script crossorigin="anonymous" async="async" integrity="sha512-MYodBGd6liZ/2mltnr0svPlWs1PUoT2pNWYBSSC8Wv3xQdvYWwtQA+FsDLVRYChJM3CeYdbFA8XIjTuQBev99g==" type="application/javascript" src="https://github.githubassets.com/assets/behaviors-bootstrap-318a1d04.js"></script>
			
				<script crossorigin="anonymous" async="async" integrity="sha512-44qssMj+fXq3KdFy9Xwq1xbyF9+0lUDA4T8YID97FKD+j18CEaDsPGutuPP3frQFBwcikEViAK+3cFq5MzmQCg==" type="application/javascript" data-module-id="./contributions-spider-graph.js" data-src="https://github.githubassets.com/assets/contributions-spider-graph-e38aacb0.js"></script>
				<script crossorigin="anonymous" async="async" integrity="sha512-EOcKJC66ZRh9HbNcX4OaUtWMBVt6SEC3qOFS0QOam5vJQ1OD1sdHNk/Pns/CboOmqzrtBDObn7Cj06879tEsog==" type="application/javascript" data-module-id="./drag-drop.js" data-src="https://github.githubassets.com/assets/drag-drop-10e70a24.js"></script>
				<script crossorigin="anonymous" async="async" integrity="sha512-+Hjt/FjfpR/rJbajZ9UxwAO/2VXMI24zoPWhrAH2xdZ8tiA/vfx79lO9G7VxZ5D6m75f1CxgkFMzLMYnEjiKJw==" type="application/javascript" data-module-id="./jump-to.js" data-src="https://github.githubassets.com/assets/jump-to-f878edfc.js"></script>
				<script crossorigin="anonymous" async="async" integrity="sha512-K7RUEyP4UVYOJabNo3qgNplXBsTCiPLkg3yVL55nKejeSITiuTDuWrK8sQH+AQQp86rkpg9R5fmkI6khIthYSQ==" type="application/javascript" data-module-id="./profile-pins-element.js" data-src="https://github.githubassets.com/assets/profile-pins-element-2bb45413.js"></script>
				<script crossorigin="anonymous" async="async" integrity="sha512-JXSmOrOQXof4xz7y+engxtqrugUopipC5LwEmsfxit4PlVe48UECBUCLuujjIADm1kjb2f/9/azX+qNspSy90w==" type="application/javascript" data-module-id="./randomColor.js" data-src="https://github.githubassets.com/assets/randomColor-2574a63a.js"></script>
				<script crossorigin="anonymous" async="async" integrity="sha512-FOUgzyCYz3T1et4Stcl3MeKUX3mZkQcsMsTQDgBj6/CtW3HrwyGMaCeXGyhSjTGibphNptgZKgDNkvL+O+2uYw==" type="application/javascript" data-module-id="./sortable-behavior.js" data-src="https://github.githubassets.com/assets/sortable-behavior-14e520cf.js"></script>
				<script crossorigin="anonymous" async="async" integrity="sha512-QKhsjzAxp13DAplN2bVwDh4Trwo6czodpXFz1mdHBPFQcHOUQyyEYBWPuR/YIc/NsjCQlv+hrI3cXPTdSxflgw==" type="application/javascript" data-module-id="./tweetsodium.js" data-src="https://github.githubassets.com/assets/tweetsodium-40a86c8f.js"></script>
				<script crossorigin="anonymous" async="async" integrity="sha512-ni1ZrTISKbKBcJ4D0ZhvZiku41hS6MyWSViGSNtD2hCRAWWgsog+aO9VKHTz4EH5+GVPi/aq6HmcdsOAhyIz3g==" type="application/javascript" data-module-id="./user-status-submit.js" data-src="https://github.githubassets.com/assets/user-status-submit-9e2d59ad.js"></script>
			
			<script crossorigin="anonymous" async="async" integrity="sha512-hZOSVsBiM8tO8FUtsU6UWWy23nhjGyyxS8/TVOcS+k57KaQEJ7SnRc0oU42Bhs1/2M5XcSEncNK4t9vU1tKNEQ==" type="application/javascript" src="https://github.githubassets.com/assets/repositories-bootstrap-85939256.js"></script>
	<script crossorigin="anonymous" async="async" integrity="sha512-1UjozWujLvZir+3i8OEl8L7+0k+s6p+IxIWDndsCzCpKzlHlZeXc7s0nqMTmiqkcYLziRiP4utRWLogteeadZA==" type="application/javascript" src="https://github.githubassets.com/assets/github-bootstrap-d548e8cd.js"></script>
		<div class="js-stale-session-flash flash flash-warn flash-banner" hidden
			>
			<svg class="octicon octicon-alert" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z"></path></svg>
			<span class="js-stale-session-flash-signed-in" hidden>You signed in with another tab or window. <a href="">Reload</a> to refresh your session.</span>
			<span class="js-stale-session-flash-signed-out" hidden>You signed out in another tab or window. <a href="">Reload</a> to refresh your session.</span>
		</div>
		<template id="site-details-dialog">
		<details class="details-reset details-overlay details-overlay-dark lh-default text-gray-dark hx_rsm" open>
			<summary role="button" aria-label="Close dialog"></summary>
			<details-dialog class="Box Box--overlay d-flex flex-column anim-fade-in fast hx_rsm-dialog hx_rsm-modal">
				<button class="Box-btn-octicon m-0 btn-octicon position-absolute right-0 top-0" type="button" aria-label="Close dialog" data-close-dialog>
					<svg class="octicon octicon-x" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path></svg>
				</button>
				<div class="octocat-spinner my-6 js-details-dialog-spinner"></div>
			</details-dialog>
		</details>
	</template>
	
		<div class="Popover js-hovercard-content position-absolute" style="display: none; outline: none;" tabindex="0">
		<div class="Popover-message Popover-message--bottom-left Popover-message--large Box box-shadow-large" style="width:360px;">
		</div>
	</div>
	
	
		</body>
	</html>
	"###;
	doc.parse(html);
	/*let tag = r#"
			<div class="hello good" readonly></div>
		"#;
	doc.parse(tag);*/
}
