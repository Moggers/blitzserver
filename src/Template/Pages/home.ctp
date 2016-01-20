<?php
/**
 * DO I HAVE TO KEEP THIS? LOL IDK
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
use Cake\Cache\Cache;
use Cake\Core\Configure;
use Cake\Datasource\ConnectionManager;
use Cake\Error\Debugger;
use Cake\Network\Exception\NotFoundException;

$this->layout = false;

if (!Configure::read('debug')):
    throw new NotFoundException();
endif;

$cakeDescription = 'One day this wont be so ugly';
?>
<!DOCTYPE html>
<html>
<head>
    <?= $this->Html->charset() ?>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>
        <?= $cakeDescription ?>
    </title>
    <?= $this->Html->meta('icon') ?>
    <?= $this->Html->css('base.css') ?>
    <?= $this->Html->css('cake.css') ?>
</head>
<body class="home">
    <header>
        <div class="header-image">
            <?= $this->Html->image( 'dom4.png' ) ?>
            <h1>Just fuck my shit up fam</h1>
        </div>
    </header>
    <div id="content">
        <div class="row">
            </div>
            <div class="columns large-5 platform checks">
				<li><?= $this->Html->link(__('Request New Match'), [ 'controller' => 'Matches', 'action' => 'add']) ?></li>
				<li><?= $this->Html->link(__('Show Matches'), [ 'controller' => 'Matches', 'action' => 'index']) ?></li>
            </div>
			<div class="columns large-5 filesystem checks">
				<li><?= $this->Html->link(__('Upload Map'), [ 'controller' => 'Maps', 'action' => 'add']) ?></li>
				<li><?= $this->Html->link(__('Show Maps'), [ 'controller' => 'Maps', 'action' => 'index']) ?></li>
			</div>
        </div>
        <hr/>
    </div>
    <footer>
    </footer>
</body>
</html>
