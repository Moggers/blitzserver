<div class="mods form large-12 medium-8 columns content">
    <?= $this->Form->create($mod) ?>
    <fieldset>
        <legend><?= __('Edit Mod') ?></legend>
        <?php
            echo $this->Form->input('name');
            echo $this->Form->input('icon');
            echo $this->Form->input('version');
            echo $this->Form->input('description');
        ?>
    </fieldset>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
