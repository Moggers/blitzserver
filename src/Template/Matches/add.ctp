<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('List Matches'), ['action' => 'index']) ?></li>
    </ul>
</nav>
<div class="matches form large-9 medium-8 columns content">
    <?= $this->Form->create($match) ?>
    <fieldset>
        <legend><?= __('Add Match') ?></legend>
        <?php
            echo $this->Form->input('map');
            echo $this->Form->input('age');
        ?>
    </fieldset>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
