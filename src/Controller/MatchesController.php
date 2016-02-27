<?php
namespace App\Controller;

use App\Controller\AppController;
use Cake\Controller\Component\CookieComponent;

/**
 * Matches Controller
 *
 * @property \App\Model\Table\MatchesTable $Matches
 */
class MatchesController extends AppController
{

	/**
	 * Paginator
	 */

	/**
	 * Index method
	 *
	 * @return void
	 */
	public function index()
	{
		$this->paginate = [
			'contain' => ['Maps', 'Nations', 'Turns' ]
		];

		if( $this->request->query('layout') == 'false' ) {
			$this->viewBuilder()->layout( false );
		}
		$this->set('matches', $this->paginate($this->Matches));
		$this->set('_serialize', ['matches']);
	}

	/**
	 * SCHEDULE MEMES
	 */
	public function weekschedule( $id = null )
	{
		$match = $this->Matches->get($id );
		if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
			if( $this->request->is('post') || $this->request->is('put')) {
				$match->day = $this->request->data['day'];
				$match->hour = $this->request->data['hour'];
				if( $this->Matches->save($match)) {
					$this->Flash->success(__('Schedule updated'));
				} else {
					$this->Flash->error('Failed to update schedule');
				}
			} else { die( pr($this) ); }
			die( json_encode( [ 'status' => 1, 'id' => $match->id ] ) );
		} else {
			die( json_encode( [ 'status' => 0, 'id' => $match->id ] ) );
		}
	}

	public function hostinterval( $id = null )
	{
		$match = $this->Matches->get($id );
		if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
			if( $this->request->is('post') || $this->request->is('put')) {
				$match->hostinterval = $this->request->data['hour'] * 60 + $this->request->data['minute'];
				if( $this->Matches->save($match)) {
					$this->Flash->success(__('Schedule updated'));
				} else {
					$this->Flash->error('Failed to update schedule');
				}
			} else { die( pr($this) ); }
			die( json_encode( [ 'status' => 1, 'id' => $match->id ] ) );
		} else {
			die( json_encode( [ 'status' => 0, 'id' => $match->id ] ) );
		}
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
				'contain' => ['Maps', 'Nations', 'Mods', 'Turns']
		]);
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
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
			$match = $this->Matches->patchEntity($match, $this->request->data, ['associated' => ['Mods']]);
			if( isset($_COOKIE['password']) && $_COOKIE['password'] != '' ){
				if( $match->name != '' ) 
					$match->name = str_replace( ' ', '_', $match->name );
				$match->status = 0;
				$match->password = $_COOKIE['password'];
				$match->port = 0;
				if ($this->Matches->save($match)) {
					$this->Flash->success(__('The match has been requested.'));
					die( json_encode( [ 'status' => 1, 'id' => $match->id ] ) );
				} else {
					$this->Flash->error(__('The match could not be saved. Please, try again.'));
				}
			} else {
				if ($this->request->is('ajax')) {
					die( json_encode( [ 'status' => 0, 'id' => $match->id ] ) );
				}
			}
		}
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$this->set('modsfull', $this->paginate($this->Matches->Mods->find()));
		$this->set('mods', $this->Matches->Mods->find('list') );
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
			if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
				$match = $this->Matches->patchEntity($match, $this->request->data);
				$match->needsrestart = 1;
				if ($this->Matches->save($match)) {
					$this->Flash->success(__('The match has been saved. The server will now restart to apply your changes'));
					return $this->redirect(['action' => 'view', $match->id]);
				} else {
					$this->Flash->error(__('The match could not be saved. Please, try again.'));
				}
			} else {
				$this->Flash->error(__('Incorrect password :)'));
			}
		}
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$this->set('_serialize', ['match']);
		return $this->redirect(['action' => 'view', $match->id]);
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
			// Get us out of here
			$this->Flash->error(__('Starting matches through this interface is currently broken. Please enable Client Start, and begin the match from inside the game'));
			return $this->redirect(['action' => 'index']);
			$match = $this->Matches->patchEntity($match, $this->request->data);

			// Once I fix this bullshit
			if( $match->playerstring == 0 ) {
				$this->Flash->error( "Can't start a game without players" );
				return $this->redirect(['action' => 'view', $match->id]);
			}
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
			if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
				$match = $this->Matches->patchEntity($match, $this->request->data);
				$match->status = -1;
				if ($this->Matches->save($match)) {
					$this->Flash->success(__('The match has been marked for death and should disappear shortly'));
					return $this->redirect(['action' => 'index']);
				} else {
					$this->Flash->error(__('Oh fuck'));
				}
			} else {
				$this->Flash->error(__('Incorrect password'));
				return $this->redirect(['action' => 'index']);
			}
		}
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$this->set('_serialize', ['match']);
		return $this->redirect(['action' => 'index']);
	}

	public function removePlayer( $id = null )
	{
		if( $this->request->is(['patch', 'get', 'put']) ) {
			$matchnation = $this->Matches->Nations->Matchnations->get($id, [
					'contain' => []
			]);
			$match = $this->Matches->get( $matchnation->match_id, [
					'contain' => []
			]);
			if( isset( $_COOKIE['password'] )&& $match->checkPassword( $_COOKIE['password'])){
				if( $match->status !== 3 ) {
					$matchnation->markdelete = 1;
					if( $this->Matches->Nations->Matchnations->save($matchnation) ) {
						$this->Flash->success(__('NOTE: Due to limitations in Dom4\'s server CLI, removing a player requires the server to be restarted, and thus, all players will have to reconnect.'));
					} else {
						$this->Flash->error(__('Hamlet: O fuck') );
					}
				} else {
					$this->Flash->error(__('Can\'t remove players from an active game' ) );
				}
			} else {
				$this->Flash->error(__('Incorrect password' ) );
			}
		}
		return $this->redirect(['action' => 'view', $matchnation->match_id]);
	}

	public function mapview( $id = null, $turnid=null )
	{
		$match = $this->Matches->get($id, [
				'contain' => ['Maps', 'Nations', 'Mods', 'Turns']
		]);
		if( $turnid == null ) {
			$turnid = $match->turn->tn-1;
		}
		$this->set('match', $match);
		$this->set('_serialize', ['match']);
		$this->set('turnid', $turnid);
	}
}
