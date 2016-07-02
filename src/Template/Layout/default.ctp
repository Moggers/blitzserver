<?php
/**
 * CakePHP(tm) : Rapid Development Framework (http://cakephp.org)
 * Copyright (c) Cake Software Foundation, Inc. (http://cakefoundation.org)
 *
 * Licensed under The MIT License
 * For full copyright and license information, please see the LICENSE.txt
 * Redistributions of files must retain the above copyright notice.
 *
 * @copyright     Copyright (c) Cake Software Foundation, Inc. (http://cakefoundation.org)
 * @link          http://cakephp.org CakePHP(tm) Project
 * @since         0.10.0
 * @license       http://www.opensource.org/licenses/mit-license.php MIT License
 */

$cakeDescription = 'Blitzserver';
?>
<!DOCTYPE html>
<html>
<head>
    <?= $this->Html->charset() ?>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>
        <?= "Blitzserver Alpha" ?>
    </title>
    <?= $this->Html->meta('icon') ?>

    <?= $this->fetch('meta') ?>
    <?= $this->fetch('css') ?>
    <?= $this->fetch('script') ?>


	<?= $this->Html->css('remodal.css'); ?>
	<?= $this->Html->css('remodal-default-theme.css'); ?>
	<?= $this->Html->css('shared.css'); ?>
	<?= $this->Html->css('modimage.css'); ?>
	<?= $this->Html->css('jquery-ui.min.css'); ?>
	<?= $this->Html->css('jquery-ui.structure.min.css'); ?>
	<?= $this->Html->css('jquery-ui.theme.min.css'); ?>
	<?= $this->Html->css('jquery-ui-slider-pips.css'); ?>
	<?= $this->Html->css('https://maxcdn.bootstrapcdn.com/bootstrap/3.3.6/css/bootstrap.min.css'); ?>


	<?= $this->Html->script('https://code.jquery.com/jquery-2.2.0.min.js'); ?>
	<?= $this->Html->script('remodal.js'); ?>
	<?= $this->Html->script('jquery.form.js'); ?>
	<?= $this->Html->script('js.cookie.js'); ?>
	<?= $this->Html->script('jquery-ui.js'); ?>
	<?= $this->Html->script('jquery-ui-slider-pips.js'); ?>
	<?= $this->Html->script('moment.js'); ?>
	<?= $this->Html->script('moment-timezone-with-data-2010-2020.min.js'); ?>
	<?= $this->Html->script('pixi.min.js'); ?>
	<?= $this->Html->script('voronoi.js'); ?>
	<?= $this->Html->script('moment-precise-range.js'); ?>
	<?= $this->Html->script('https://maxcdn.bootstrapcdn.com/bootstrap/3.3.6/js/bootstrap.min.js'); ?>
	
	<script type='text/javascript'>
		$(document).ready( function() {
			$('#password').val( Cookies.get('password') );
			$(document).on('confirmation', '#passwordmodal', function() {
				console.log( 'wewlad' );
				$('#password').val($('#remodal-password').val());
				Cookies.set('password', $('#password').val());
			});
			$('#password').on('change', function(event) {
				Cookies.set('password', $('#password').val());
			});
		});

	</script>
</head>
<body>
	<nav class="navbar navbar-default">
		<div class="container-fluid">
			<div class="navbar-header">
				<button type="button" class="navbar-toggle collapsed" data-toggle="collapse" data-target="#bs-navbar-collapse-1" aria-expanded="false">
					<span class="sr-only">Toggle navigation</span>
					<span class="icon-bar"></span>
					<span class="icon-bar"></span>
					<span class="icon-bar"></span>
				</button>
				<a class="navbar-brand" href="#">Blitzserver</a>
			</div>
			<div class="collapse navbar-collapse" id="bs-navbar-collapse-1">
				<ul class="nav navbar-nav navbar-left">
					<li class="dropdown">
						<a href="matches" class="dropdown-toggle" data-toggle="dropdown" role="button" aria-haspopup="true" aria-expanded="false">Matches <span class="caret"></span></a>
						<ul class="dropdown-menu">
						 <li><a href="/matches/add">Create </a></li>
						 <li><a href="/matches/index">List </a></li>
						</ul>
					</li>
					<li class="dropdown">
						<a href="matches" class="dropdown-toggle" data-toggle="dropdown" role="button" aria-haspopup="true" aria-expanded="false">Maps <span class="caret"></span></a>
						<ul class="dropdown-menu">
						 <li><a href="/maps/add">Upload </a></li>
						 <li><a href="/maps/index">List </a></li>
						</ul>
					</li>
					<li class="dropdown">
						<a href="matches" class="dropdown-toggle" data-toggle="dropdown" role="button" aria-haspopup="true" aria-expanded="false">Mods <span class="caret"></span></a>
						<ul class="dropdown-menu">
						 <li><a href="/mods/add">Upload </a></li>
						 <li><a href="/mods/index">List </a></li>
						</ul>
					</li>
				</ul>
				<ul class="nav navbar-nav navbar-right">
					<li><input class="form-control" type="password" name="password" id="password" placeholder="Password"/></li>
					<li><a href="https://gitreports.com/issue/Moggers/blitzserver">Bug Report</a></lI>
					<li><a href="https://github.com/moggers/blitzserver">Github</a></lI>
				</ul>
			</div>
		</div>
	</nav>
    <?= $this->Flash->render() ?>
    <section class="container clearfix">
        <?= $this->fetch('content') ?>
    </section>
    <footer>
    </footer>
	<div class="remodal" id='passwordmodal' data-remodal-id="modal">
		<button data-remodal-action="close" class="remodal-close"></button>
		<h1>Please Enter Password</h1>
		<?= $this->Form->input('remodal-password', ['label' => false, 'placeholder' => 'Password']); ?>
		<br />
		<button data-remodal-action="cancel" class="remodal-cancel">Cancel</button>
		<button data-remodal-action="confirm" class="remodal-confirm">Confirm</button>
	</div>
</body>
</html>
