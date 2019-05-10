#include <iostream>
#include <vector>
#include <memory>

class BalancedBTreeSet {
	class Node {
		public:
			int value, left_height, right_height;
			std::unique_ptr<Node> left, right;
			Node(int val): value(val), left_height(0), right_height(0), left(std::unique_ptr<Node>()), right(std::unique_ptr<Node>()) {}
	};
	private:
		int size;
		std::unique_ptr<Node> root;
	public:
		BalancedBTreeSet(): size(0), root(std::unique_ptr<Node>()){}
		void rotate(std::unique_ptr<Node>& parent, bool is_right) {
			if (is_right) {
				auto* child_right_ptr = parent->left->right.release();
				auto* child_ptr = parent->left.release();
				auto* parent_ptr = parent.release();
				parent.reset(child_ptr);
				parent->right.reset(parent_ptr);
				parent->right->left.reset(child_right_ptr);
				parent->right->left_height = (parent->right->left)? std::max(parent->right->left->left_height, parent->right->left->right_height) + 1 : 0;
				parent->right_height = std::max(parent->right->left_height, parent->right->right_height) + 1;
			} else {
				auto* child_left_ptr = parent->right->left.release();
				auto* child_ptr = parent->right.release();
				auto* parent_ptr = parent.release();
				parent.reset(child_ptr);
				parent->left.reset(parent_ptr);
				parent->left->right.reset(child_left_ptr);
				parent->left->right_height = (parent->left->right)? std::max(parent->left->right->left_height, parent->left->right->right_height) + 1 : 0;
				parent->left_height = std::max(parent->left->left_height, parent->left->right_height) + 1;
			}
		}
		bool insert_rec(int x, std::unique_ptr<Node>& node) {
			if (!node) {
				node.reset(new Node(x));
				return true;
			}
			if (x < node->value && insert_rec(x, node->left)) {
				node->left_height++;
			} else if (x > node->value && insert_rec(x, node->right)) {
				node->right_height++;
			}
			if (node->left_height - node->right_height > 1) {
				if (node->left->left_height > node->left->right_height) {
					rotate(node, true);
				} else {
					rotate(node->left, false);
					rotate(node, true);
				}
			} else if (node->right_height - node->left_height > 1) {
				if (node->right->right_height > node->right->left_height) {
					rotate(node, false);
				} else {
					rotate(node->right, true);
					rotate(node, false);
				}
			}
			return node->left_height != node->right_height;
		}
		void insert(int x) {
			insert_rec(x, root);
		}
		bool remove_rec(int x, std::unique_ptr<Node>& node) {
			if (!node) {
				return false;
			}
			if (x < node->value && remove_rec(x, node->left)) {
				node->left_height--;
			} else if (x > node->value && remove_rec(x, node->right)) {
				node->right_height--;
			} else if (x == node->value) {
				
			}
		}
};