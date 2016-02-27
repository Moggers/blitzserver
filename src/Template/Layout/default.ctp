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
        <?= $cakeDescription ?>:
        <?= "Blitzserver Alpha" . $this->fetch('title') ?>
    </title>
    <?= $this->Html->meta('icon') ?>

    <?= $this->Html->css('base.css') ?>
    <?= $this->Html->css('cake.css') ?>

    <?= $this->fetch('meta') ?>
    <?= $this->fetch('css') ?>
    <?= $this->fetch('script') ?>


	<?= $this->Html->css('remodal.css'); ?>
	<?= $this->Html->css('remodal-default-theme.css'); ?>
	<?= $this->Html->css('shared.css'); ?>

	<?= $this->Html->script('https://code.jquery.com/jquery-2.2.0.min.js'); ?>
	<?= $this->Html->script('remodal.js'); ?>
	<?= $this->Html->script('jquery.form.js'); ?>
	<?= $this->Html->script('js.cookie.js'); ?>
	<?= $this->Html->script('moment.js'); ?>
	<?= $this->Html->script('moment-timezone-with-data-2010-2020.min.js'); ?>
	<?= $this->Html->script('pixi.min.js'); ?>
	
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
    <nav class="top-bar expanded" data-topbar role="navigation">
        <ul class="title-area large-3 medium-4 columns">
            <li class="name">
                <h1><a href="">Blitzserver <sup>Alpha</sup></a></h1>
            </li>
        </ul>
        <section class="top-bar-section">
            <ul class="left">
				<li><?= $this->Html->link(__('Show Matches'), ['controller' => 'Matches', 'action' => 'index']) ?></li>
				<li><?= $this->Html->link(__('New Match'), ['controller' => 'Matches', 'action' => 'add']) ?></li>
				<li><?= $this->Html->link(__('Show Maps'), ['controller' => 'Maps', 'action' => 'index']) ?></li>
				<li><?= $this->Html->link(__('Upload Map'), ['controller' => 'Maps', 'action' => 'add']) ?></li>
				<li><?= $this->Html->link(__('Show Mods'), ['controller' => 'Mods', 'action' => 'index']) ?></li>
				<li><?= $this->Html->link(__('Upload Mod'), ['controller' => 'Mods', 'action' => 'add']) ?></li>
				<li><?= $this->Html->link(__('Bug Report/Feature Request'), 'https://gitreports.com/issue/Moggers/blitzserver') ?></li>
            </ul>
			<ul class="right">
				<li> <?= $this->Form->input('password', ['label' => false, 'id' => 'password', 'placeholder' => 'Password']); ?> </li>
			</ul>
        </section>
		</section>
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
		<button data-remodal-action="cancel" class="remodal-cancel">Nah think oi'll come back later, eh?</button>
		<button data-remodal-action="confirm" class="remodal-confirm">Aight m8, giv 'er a go</button>
	</div>
</body>
</html>
