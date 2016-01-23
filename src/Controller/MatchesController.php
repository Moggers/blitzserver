<?php
namespace App\Controller;

use App\Controller\AppController;

/**
 * Matches Controller
 *
 * @property \App\Model\Table\MatchesTable $Matches
 */
class MatchesController extends AppController
{

    /**
     * Index method
     *
     * @return void
     */
    public function index()
    {
        $this->paginate = [
            'contain' => ['Maps']
        ];

		if( $this->request->query('layout') == 'false' ) {
			$this->viewBuilder()->layout( false );
		}
        $this->set('matches', $this->paginate($this->Matches));
        $this->set('_serialize', ['matches']);
    }

    /**
     * View method
     *
     * @param string|null $id Match id.
     * @return void
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function view($id = null)
    {
        $match = $this->Matches->get($id, [
            'contain' => ['Maps']
        ]);
        $this->set('match', $match);
        $this->set('_serialize', ['match']);
    }

    /**
     * Add method
     *
     * @return void Redirects on successful add, renders view otherwise.
     */
    public function add()
    {
        $match = $this->Matches->newEntity();
        if ($this->request->is('post')) {
            $match = $this->Matches->patchEntity($match, $this->request->data);
			$match->status = 0;
			$match->port = 0;
            if ($this->Matches->save($match)) {
                $this->Flash->success(__('The match has been saved.'));
                return $this->redirect(['action' => 'index']);
            } else {
                $this->Flash->error(__('The match could not be saved. Please, try again.'));
            }
        }
        $maps = $this->Matches->Maps->find('list', ['limit' => 200]);
        $this->set(compact('match', 'maps'));
        $this->set('_serialize', ['match']);
    }

    /**
     * Edit method
     *
     * @param string|null $id Match id.
     * @return void Redirects on successful edit, renders view otherwise.
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function edit($id = null)
    {
        $match = $this->Matches->get($id, [
            'contain' => []
        ]);
        if ($this->request->is(['patch', 'post', 'put'])) {
            $match = $this->Matches->patchEntity($match, $this->request->data);
            if ($this->Matches->save($match)) {
                $this->Flash->success(__('The match has been saved.'));
                return $this->redirect(['action' => 'index']);
            } else {
                $this->Flash->error(__('The match could not be saved. Please, try again.'));
            }
        }
        $maps = $this->Matches->Maps->find('list', ['limit' => 200]);
        $this->set(compact('match', 'maps'));
        $this->set('_serialize', ['match']);
    }

    /**
     * Delete method
     *
     * @param string|null $id Match id.
     * @return \Cake\Network\Response|null Redirects to index.
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function delete($id = null)
    {
        $this->request->allowMethod(['post', 'delete']);
        $match = $this->Matches->get($id);
        if ($this->Matches->delete($match)) {
            $this->Flash->success(__('The match has been deleted.'));
        } else {
            $this->Flash->error(__('The match could not be deleted. Please, try again.'));
        }
        return $this->redirect(['action' => 'index']);
    }

	public function start($id = null)
	{
        $match = $this->Matches->get($id, [
            'contain' => []
        ]);
        if ($this->request->is(['patch', 'get', 'put'])) {
            $match = $this->Matches->patchEntity($match, $this->request->data);
			$match->status = 2;
            if ($this->Matches->save($match)) {
                $this->Flash->success(__('The match has been started.'));
                return $this->redirect(['action' => 'index']);
            } else {
                $this->Flash->error(__('Oh fuck'));
            }
        }
        $maps = $this->Matches->Maps->find('list', ['limit' => 200]);
        $this->set(compact('match', 'maps'));
        $this->set('_serialize', ['match']);
	}
	public function destroy($id = null)
	{
        $match = $this->Matches->get($id, [
            'contain' => []
        ]);
        if ($this->request->is(['patch', 'get', 'put'])) {
            $match = $this->Matches->patchEntity($match, $this->request->data);
			$match->status = -1;
            if ($this->Matches->save($match)) {
                $this->Flash->success(__('The match has been marked for death and should disappear shortly'));
                return $this->redirect(['action' => 'index']);
            } else {
                $this->Flash->error(__('Oh fuck'));
            }
        }
        $maps = $this->Matches->Maps->find('list', ['limit' => 200]);
        $this->set(compact('match', 'maps'));
        $this->set('_serialize', ['match']);
	}
}
