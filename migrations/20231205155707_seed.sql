-- Inserir dados na tabela 'usuarios'
INSERT INTO `usuarios` (`nome`, `email`, `senha`) VALUES
('João Silva', 'joao.silva@example.com', 'senha123'),
('Maria Oliveira', 'maria.oliveira@example.com', 'senha321'),
('Carlos Santos', 'carlos.santos@example.com', 'senha456');

-- Inserir dados na tabela 'comunidades'
INSERT INTO `comunidades` (`nome`, `desc`) VALUES
('Tecnologia', 'Discussões sobre as últimas novidades em tecnologia e gadgets'),
('Culinária', 'Compartilhamento de receitas e dicas de culinária'),
('Viagens', 'Experiências e dicas de viagem pelo mundo');

-- Inserir dados na tabela 'tags'
INSERT INTO `tags` (`nome`, `comunidade_id`) VALUES
('Smartphones', 1),
('Receitas', 2),
('Destinos', 3);

-- Inserir dados na tabela 'inscricoes'
INSERT INTO `inscricoes` (`usuario_id`, `comunidade_id`) VALUES
(1, 1),
(2, 2),
(3, 3);

-- Inserir dados na tabela 'posts'
INSERT INTO `posts` (`titulo`, `body`, `tag_id`, `usuario_id`, `comunidade_id`) VALUES
('Novo Smartphone no Mercado', 'Confira as últimas novidades do mundo dos smartphones.', 1, 1, 1),
('Receita de Bolo de Chocolate', 'Aprenda a fazer um delicioso bolo de chocolate.', 2, 2, 2),
('Minha Viagem à Tailândia', 'Compartilhando minhas experiências de viagem na Tailândia.', 3, 3, 3);

-- Inserir dados nas tabelas de avaliações (exemplos)
INSERT INTO `usuarios_avaliam_posts` (`post_id`, `usuario_id`, `gostou`) VALUES
(1, 2, TRUE),
(2, 1, TRUE);

-- Inserir dados na tabela 'comentarios'
INSERT INTO `comentarios` (`body`, `post_id`, `usuario_id`) VALUES
('Muito interessante este artigo sobre smartphones!', 1, 2),
('Adorei a receita, vou tentar fazer em casa!', 2, 1);

INSERT INTO `usuarios_avaliam_comentarios` (`comentario_id`, `usuario_id`, `gostou`) VALUES
(1, 3, TRUE);

INSERT INTO `tags` (`nome`, `comunidade_id`) VALUES
('Inteligência Artificial', 1),
('Jogos Eletrônicos', 1);

INSERT INTO `posts` (`titulo`, `body`, `tag_id`, `usuario_id`, `comunidade_id`) VALUES
('Avanços em Inteligência Artificial', 'Discussão sobre os últimos avanços na área de IA e como eles estão moldando o futuro.', (SELECT id FROM tags WHERE nome = 'Inteligência Artificial'), 1, 1);

INSERT INTO `posts` (`titulo`, `body`, `tag_id`, `usuario_id`, `comunidade_id`) VALUES
('A Evolução dos Jogos Eletrônicos', 'Uma análise da evolução dos jogos desde os clássicos até os modernos jogos de realidade virtual.', (SELECT id FROM tags WHERE nome = 'Jogos Eletrônicos'), 2, 1);

INSERT INTO `posts` (`titulo`, `body`, `usuario_id`, `comunidade_id`) VALUES
('Tendências de Gadgets em 2023', 'Explorando as principais tendências e novidades no mundo dos gadgets para o ano de 2023.', 3, 1);
