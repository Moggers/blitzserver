<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('List Matches'), ['action' => 'index']) ?></li>
        <li><?= $this->Html->link(__('List Maps'), ['controller' => 'Maps', 'action' => 'index']) ?></li>
        <li><?= $this->Html->link(__('New Map'), ['controller' => 'Maps', 'action' => 'add']) ?></li>
    </ul>
</nav>
<div class="matches form large-9 medium-8 columns content">
    <?= $this->Form->create($match) ?>
    <fieldset>
        <legend><?= __('Add Match') ?></legend>
        <?php
            echo $this->Form->input('map_id', ['options' => $maps]);
            echo $this->Form->input('age');
			echo $this->Form->input('name');
        ?>
    </fieldset>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
