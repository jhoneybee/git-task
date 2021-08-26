# 这是对 git cherry-pick 的命令封装

## git-task tags new 

用来创建一个 tag 标记当期的 任务信息

执行命令的格式如下

```
git-task tags new my-tags
```

## git-task tags ls

查看当前的 tags 信息

```
git-task tags ls
```


## git-task task new 

创建一个开发任务

```
git-task task new my-task
```

## git-task task ls 

查看当前的开发任务

```
git-task task ls 
```

## git-task commit 

提交当期的 commit 修改


```
git-task commit my-task "完成的代码修改"
```

## git-task cherry-pick 

将这个任务的代码全部 cherry-pick  到当前分支

例如 

develop 分支为开发分支 
qs-uat  分支为用户测试分支
qs-prod 分支为生产分支

将代码全部提交到 qs-uat 以后， 开发想将部分的任务提交到 qs-uat 则执行如下

```

# develop 
git checkout qs-prod # 切换到qs-prod 

git-task cherry-pick my-task # 将 qs-uat 的这个修改全部合并到 qs-prod

git push # 提交到自己的分支， 发起 merge request 即可 

```

## git-task version 

查看当前的版本信息

```
git-task version 
```